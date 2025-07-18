use crate::emulator::r6502_image::Snapshot;
use crate::emulator::util::make_unique_snapshot_path;
use crate::emulator::{Bus, BusEvent, Cpu, IoEvent, MOS_6502, Opcode, RESET};
use crate::machine_config::{HostHookType, MachineInfo};
use crate::terminal_ui::acorn_host_hooks::handle_host_hook;
use crate::terminal_ui::{StopReason, TerminalChannel, TerminalEvent};
use anyhow::{Result, anyhow, bail};
use cursive::backends::crossterm::crossterm::event::{Event, poll, read};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread::spawn;
use std::time::Duration;

// TBD: This is ugly but it'll work for now
pub struct Runner<'a> {
    pub cpu: &'a mut Cpu<'a>,
    pub bus_rx: Receiver<BusEvent>,
    pub terminal_channel: TerminalChannel,
    pub io_tx: Sender<IoEvent>,
    pub stop_after: Option<u64>,
    pub machine_info: MachineInfo,
    pub bus: &'a Bus,
}

impl Runner<'_> {
    pub fn run(self) -> Result<StopReason> {
        let handle = spawn(move || {
            Self::event_loop(&self.terminal_channel.rx, &self.io_tx).expect("Must succeed");
        });
        let stop_reason =
            Self::do_steps(self.cpu, &self.bus_rx, self.stop_after, &self.machine_info)?;
        _ = self.terminal_channel.tx.send(TerminalEvent::Shutdown);
        if handle.join().is_err() {
            bail!("internal error: most likely a thread panicked; check r6502.log for more info")
        }
        if !self.bus.stop() {
            bail!("internal error: most likely a thread panicked; check r6502.log for more info")
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

        loop {
            cpu.step_with_monitor_callbacks();

            match bus_rx.try_recv() {
                Ok(BusEvent::UserBreak) => {
                    return Ok(StopReason::UserBreak {
                        total_cycles: cpu.total_cycles,
                    });
                }
                Ok(BusEvent::Reset) => {
                    jmp_ind.execute_word(cpu, RESET);
                }
                Ok(BusEvent::Snapshot) => {
                    let snapshot = Snapshot::new(cpu);
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

            if let Some(host_hook) = &machine_info.machine.host_hook {
                if cpu.reg.pc == host_hook.addr {
                    match host_hook.r#type {
                        HostHookType::Acorn => {
                            handle_host_hook(cpu)?;
                            let return_addr = cpu.pull_word().wrapping_add(1);
                            cpu.reg.pc = return_addr;
                        }
                    }
                }
            }
        }
    }

    fn event_loop(terminal_rx: &Receiver<TerminalEvent>, io_tx: &Sender<IoEvent>) -> Result<()> {
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
                _ = io_tx.send(IoEvent::Input(event));
            }
        }

        Ok(())
    }
}
