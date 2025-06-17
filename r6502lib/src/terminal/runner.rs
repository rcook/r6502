use crate::emulator::r6502_image::Image;
use crate::emulator::util::make_unique_snapshot_path;
use crate::emulator::{Bus, BusEvent, Cpu, Opcode, PiaEvent, MOS_6502, RESET};
use crate::machine_config::MachineInfo;
use crate::terminal::{RawMode, TerminalChannel, TerminalEvent};
use anyhow::{anyhow, Result};
use cursive::backends::crossterm::crossterm::event::{poll, read, Event};
use std::process::exit;
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
    pub cycles: bool,
}

impl<'a> Runner<'a> {
    pub fn run(self) -> Result<()> {
        let handle = spawn(move || {
            Self::event_loop(&self.terminal_channel.rx, &self.pia_tx).expect("Must succeed");
        });

        let jmp_ind = MOS_6502
            .get_op_info(&Opcode::JmpInd)
            .ok_or_else(|| anyhow!("JMP_IND must exist"))?
            .clone();

        let rts = MOS_6502
            .get_op_info(&Opcode::Rts)
            .ok_or_else(|| anyhow!("RTS must exist"))?
            .clone();

        let mut stopped_after_requested_cycles = false;
        'outer: loop {
            while self.cpu.step() {
                match self.bus_rx.try_recv() {
                    Ok(BusEvent::UserBreak) => {
                        break 'outer;
                    }
                    Ok(BusEvent::Reset) => {
                        jmp_ind.execute_word(self.cpu, RESET);
                    }
                    Ok(BusEvent::Snapshot) => {
                        let snapshot = Image::new_snapshot(self.cpu);
                        let snapshot_path = make_unique_snapshot_path()?;
                        snapshot.write(&snapshot_path)?;
                    }
                    Err(TryRecvError::Disconnected | TryRecvError::Empty) => {}
                }

                if let Some(stop_after) = self.stop_after {
                    if self.cpu.total_cycles >= stop_after {
                        stopped_after_requested_cycles = true;
                        break 'outer;
                    }
                }

                if let Some(halt_addr) = self.machine_info.machine.halt_addr {
                    if self.cpu.reg.pc == halt_addr {
                        break 'outer;
                    }
                }

                if let Some(write_char_addr) = self.machine_info.machine.write_char_addr {
                    if self.cpu.reg.pc == write_char_addr {
                        print!("{}", self.cpu.reg.a as char);
                        rts.execute_no_operand(self.cpu);
                    }
                }
            }
        }

        _ = self.terminal_channel.tx.send(TerminalEvent::Shutdown);
        _ = handle.join();

        self.bus.stop();

        // If program hits BRK return contents of A as exit code, otherwise 0.
        let code = if stopped_after_requested_cycles {
            0
        } else {
            self.cpu.reg.a as i32
        };

        if self.cycles {
            if stopped_after_requested_cycles {
                println!(
                    "Stopped after {cycles} cycles with exit code {code}",
                    cycles = self.cpu.total_cycles
                );
            } else {
                println!(
                    "Completed after {cycles} total cycles with exit code {code}",
                    cycles = self.cpu.total_cycles
                );
            }
        }

        exit(code)
    }

    fn event_loop(terminal_rx: &Receiver<TerminalEvent>, pia_tx: &Sender<PiaEvent>) -> Result<()> {
        fn try_read_event() -> Result<Option<Event>> {
            if poll(Duration::from_millis(100))? {
                Ok(Some(read()?))
            } else {
                Ok(None)
            }
        }

        let raw_mode = RawMode::new()?;

        loop {
            match terminal_rx.try_recv() {
                Ok(TerminalEvent::Shutdown) | Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }

            if let Some(event) = try_read_event()? {
                _ = pia_tx.send(PiaEvent::Input(event));
            }
        }

        drop(raw_mode);

        Ok(())
    }
}
