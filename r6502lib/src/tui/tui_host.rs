use crate::emulator::util::get_brk_addr;
use crate::emulator::{AddressRange, Bus, Cpu, Image, InstructionInfo, OpInfo, Opcode, MOS_6502};
use crate::machine_config::MachineInfo;
use crate::messages::State::{Halted, Running, Stepping, Stopped};
use crate::messages::{DebugMessage, IoMessage, MonitorMessage, State};
use crate::p_set;
use crate::tui::TuiMonitor;
use anyhow::{anyhow, Result};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

// TBD: Come up with a better name for this struct!
pub struct TuiHost {
    machine_info: MachineInfo,
    bus: Bus,
    debug_rx: Receiver<DebugMessage>,
    monitor_tx: Sender<MonitorMessage>,
    io_tx: Sender<IoMessage>,
}

impl TuiHost {
    pub const fn new(
        machine_info: MachineInfo,
        bus: Bus,
        debug_rx: Receiver<DebugMessage>,
        monitor_tx: Sender<MonitorMessage>,
        io_tx: Sender<IoMessage>,
    ) -> Self {
        Self {
            machine_info,
            bus,
            debug_rx,
            monitor_tx,
            io_tx,
        }
    }

    pub fn run(&self, image: &Image) -> Result<()> {
        let monitor = Box::new(TuiMonitor::new(self.monitor_tx.clone()));

        let mut cpu = Cpu::new(self.bus.view(), Some(monitor));
        image.set_initial_cpu_state(&mut cpu);

        let rti = MOS_6502
            .get_op_info(&Opcode::Rti)
            .ok_or_else(|| anyhow!("RTI must exist"))?
            .clone();

        let mut state = Stepping;
        loop {
            self.send_state(state);

            match state {
                Running => state = self.handle_running(&mut cpu, &rti),
                Stepping => state = self.handle_stepping(&mut cpu, &rti),
                Halted => state = self.handle_halted(&mut cpu),
                Stopped => break,
            }
        }

        Ok(())
    }

    fn send_state(&self, state: State) {
        _ = self.monitor_tx.send(MonitorMessage::NotifyState(state));
    }

    fn fetch_instruction(&self, cpu: &Cpu) {
        let instruction_info = InstructionInfo::fetch(cpu);
        _ = self.monitor_tx.send(MonitorMessage::BeforeExecute {
            total_cycles: cpu.total_cycles,
            reg: cpu.reg.clone(),
            instruction_info,
        });
    }

    fn handle_brk(&self, cpu: &mut Cpu, rti: &OpInfo, state: State) -> State {
        match (self.machine_info.machine.write_char_addr, get_brk_addr(cpu)) {
            (Some(write_char_addr), Some(brk_addr)) if brk_addr == write_char_addr => {
                self.io_tx
                    .send(IoMessage::WriteChar(cpu.reg.a as char))
                    .expect("Must succeed");
                rti.execute_no_operand(cpu);
                state
            }
            _ => {
                _ = self.monitor_tx.send(MonitorMessage::NotifyInvalidBrk);
                Halted
            }
        }
    }

    fn handle_running(&self, cpu: &mut Cpu, rti: &OpInfo) -> State {
        loop {
            match self.debug_rx.try_recv() {
                Err(TryRecvError::Disconnected) => return Stopped,
                Err(TryRecvError::Empty) | Ok(_) => {}
            }

            if !cpu.step2(true) {
                let new_state = self.handle_brk(cpu, rti, Running);
                if !matches!(new_state, Stepping) {
                    return new_state;
                }
            }

            if let Some(halt_addr) = self.machine_info.machine.halt_addr {
                if cpu.reg.pc == halt_addr {
                    return Halted;
                }
            }
        }
    }

    fn handle_stepping(&self, cpu: &mut Cpu, rti: &OpInfo) -> State {
        loop {
            self.fetch_instruction(cpu);

            match self.debug_rx.recv() {
                Err(_) => return Stopped,
                Ok(m) => match m {
                    DebugMessage::Step | DebugMessage::Break => {}
                    DebugMessage::Run => return Running,
                    DebugMessage::FetchMemory(address_range) => self.fetch_memory(&address_range),
                    DebugMessage::SetPc(addr) => self.set_pc(cpu, addr),
                    DebugMessage::Go(addr) => {
                        p_set!(cpu.reg, B, false);
                        self.set_pc(cpu, addr);
                    }
                },
            }

            if !cpu.step() {
                let new_state = self.handle_brk(cpu, rti, Stepping);
                if !matches!(new_state, Stepping) {
                    return new_state;
                }
            }

            if let Some(halt_addr) = self.machine_info.machine.halt_addr {
                if cpu.reg.pc == halt_addr {
                    return Halted;
                }
            }
        }
    }

    fn handle_halted(&self, cpu: &mut Cpu) -> State {
        loop {
            match self.debug_rx.recv() {
                Err(_) => return Stopped,
                Ok(m) => match m {
                    DebugMessage::Step | DebugMessage::Run | DebugMessage::Break => {}
                    DebugMessage::FetchMemory(address_range) => self.fetch_memory(&address_range),
                    DebugMessage::SetPc(addr) => self.set_pc(cpu, addr),
                    DebugMessage::Go(addr) => {
                        p_set!(cpu.reg, B, false);
                        self.set_pc(cpu, addr);
                        return Stepping;
                    }
                },
            }
        }
    }

    fn fetch_memory(&self, address_range: &AddressRange) {
        let snapshot = self.bus.snapshot(address_range);
        _ = self.monitor_tx.send(MonitorMessage::FetchMemoryResponse {
            address_range: address_range.clone(),
            snapshot,
        });
    }

    fn set_pc(&self, cpu: &mut Cpu, addr: u16) {
        cpu.reg.pc = addr;
        self.fetch_instruction(cpu);
    }
}
