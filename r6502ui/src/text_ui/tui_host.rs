use crate::text_ui::TuiMonitor;
use r6502core::emulator::{Bus, Cpu, InstructionInfo};
use r6502core::messages::State::{Halted, Running, Stepping, Stopped};
use r6502core::messages::{DebugMessage, MonitorMessage, State};
use r6502core::{InterruptChannel, p_get, p_set};
use r6502hw::MachineInfo;
use r6502lib::AddressRange;
use r6502lib::constants::RESET;
use r6502lib::util::make_word;
use r6502snapshot::MemoryImage;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

// TBD: Come up with a better name for this struct!
pub struct TuiHost {
    machine_info: MachineInfo,
    bus: Bus,
    debug_rx: Receiver<DebugMessage>,
    monitor_tx: Sender<MonitorMessage>,
}

impl TuiHost {
    pub const fn new(
        machine_info: MachineInfo,
        bus: Bus,
        debug_rx: Receiver<DebugMessage>,
        monitor_tx: Sender<MonitorMessage>,
    ) -> Self {
        Self {
            machine_info,
            bus,
            debug_rx,
            monitor_tx,
        }
    }

    pub fn run(&self, image: &MemoryImage) {
        let monitor = Box::new(TuiMonitor::new(self.monitor_tx.clone()));
        let interrupt_channel = InterruptChannel::new();

        let mut cpu = Cpu::new(self.bus.view(), Some(monitor), interrupt_channel.rx);
        let reset_addr_lo = cpu.bus.load(RESET);
        let reset_addr_hi = cpu.bus.load(RESET.wrapping_add(1));
        let reset_addr = make_word(reset_addr_hi, reset_addr_lo);
        let cpu_state = image.get_initial_cpu_state(reset_addr);
        cpu.set_initial_state(&cpu_state);

        let mut state = Stepping;
        loop {
            self.send_state(state);

            match state {
                Running => state = self.handle_running(&mut cpu),
                Stepping => state = self.handle_stepping(&mut cpu),
                Halted => state = self.handle_halted(&mut cpu),
                Stopped => break,
            }
        }
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

    fn handle_brk(&self) -> State {
        _ = self.monitor_tx.send(MonitorMessage::NotifyInvalidBrk);
        Halted
    }

    fn handle_running(&self, cpu: &mut Cpu) -> State {
        loop {
            match self.debug_rx.try_recv() {
                Err(TryRecvError::Disconnected) => return Stopped,
                Err(TryRecvError::Empty) | Ok(_) => {}
            }

            cpu.step();
            if p_get!(cpu.reg, I) {
                let new_state = self.handle_brk();
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

    fn handle_stepping(&self, cpu: &mut Cpu) -> State {
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

            cpu.step_with_monitor_callbacks();
            if p_get!(cpu.reg, I) {
                let new_state = self.handle_brk();
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
