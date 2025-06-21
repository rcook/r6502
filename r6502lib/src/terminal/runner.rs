use crate::emulator::r6502_image::Image;
use crate::emulator::util::make_unique_snapshot_path;
use crate::emulator::{Bus, BusEvent, Cpu, Opcode, PiaEvent, MOS_6502, RESET};
use crate::machine_config::MachineInfo;
use crate::terminal::{StopReason, TerminalChannel, TerminalEvent};
use anyhow::{anyhow, bail, Result};
use cursive::backends::crossterm::crossterm::event::{poll, read, Event};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread::spawn;
use std::time::Duration;

// TBD: This is ugly but it'll work for now
pub struct Runner<'a> {
    pub cpu: &'a mut Cpu<'a>,
    pub bus_rx: Receiver<BusEvent>,
    pub terminal_channel: TerminalChannel,
    pub pia_tx: Sender<PiaEvent>,
    pub stop_after: Option<u64>,
    pub machine_info: MachineInfo,
    pub bus: &'a Bus,
}

impl Runner<'_> {
    pub fn run(self) -> Result<StopReason> {
        let handle = spawn(move || {
            Self::event_loop(&self.terminal_channel.rx, &self.pia_tx).expect("Must succeed");
        });
        let stop_reason =
            Self::do_steps(self.cpu, &self.bus_rx, self.stop_after, &self.machine_info)?;
        _ = self.terminal_channel.tx.send(TerminalEvent::Shutdown);
        if handle.join().is_err() {
            bail!("Internal error: most likely a thread panicked; check r6502.log for more info")
        }
        if !self.bus.stop() {
            bail!("Internal error: most likely a thread panicked; check r6502.log for more info")
        }
        Ok(stop_reason)
    }

    fn do_steps(
        cpu: &mut Cpu,
        bus_rx: &Receiver<BusEvent>,
        stop_after: Option<u64>,
        machine_info: &MachineInfo,
    ) -> Result<StopReason> {
        let jmp_ind = MOS_6502
            .get_op_info(&Opcode::JmpInd)
            .ok_or_else(|| anyhow!("JMP_IND must exist"))?
            .clone();

        while cpu.step() {
            match bus_rx.try_recv() {
                Ok(BusEvent::UserBreak) => {
                    return Ok(StopReason::UserBreak {
                        total_cycles: cpu.total_cycles,
                    })
                }
                Ok(BusEvent::Reset) => {
                    jmp_ind.execute_word(cpu, RESET);
                }
                Ok(BusEvent::Snapshot) => {
                    let snapshot = Image::new_snapshot(cpu);
                    let snapshot_path = make_unique_snapshot_path()?;
                    snapshot.write(&snapshot_path)?;
                }
                Err(TryRecvError::Disconnected | TryRecvError::Empty) => {}
            }

            if let Some(stop_after) = stop_after {
                if cpu.total_cycles >= stop_after {
                    return Ok(StopReason::RequestedCyclesExecuted {
                        total_cycles: cpu.total_cycles,
                    });
                }
            }

            if let Some(halt_addr) = machine_info.machine.halt_addr {
                if cpu.reg.pc == halt_addr {
                    return Ok(StopReason::Halt {
                        total_cycles: cpu.total_cycles,
                        a: cpu.reg.a,
                    });
                }
            }
        }

        // Terminated due to unexpected software interrupt (BRK)
        Ok(StopReason::UnexpectedInterrupt {
            total_cycles: cpu.total_cycles,
        })
    }

    fn event_loop(terminal_rx: &Receiver<TerminalEvent>, pia_tx: &Sender<PiaEvent>) -> Result<()> {
        fn try_read_event() -> Result<Option<Event>> {
            if poll(Duration::from_millis(100))? {
                Ok(Some(read()?))
            } else {
                Ok(None)
            }
        }

        loop {
            match terminal_rx.try_recv() {
                Ok(TerminalEvent::Shutdown) | Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }

            if let Some(event) = try_read_event()? {
                _ = pia_tx.send(PiaEvent::Input(event));
            }
        }

        Ok(())
    }
}
