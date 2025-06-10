use crate::State::*;
use crate::{AddressRange, DebugMessage, IoMessage, MonitorMessage, State, UiMonitor};
use anyhow::{anyhow, Result};
use r6502lib::{
    p_set, Cpu, Image, InstructionInfo, Memory, OpInfo, Opcode, Os, OsEmulation, MOS_6502, OSHALT,
    OSWRCH,
};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

// TBD: Come up with a better name for this struct!
pub(crate) struct UiHost {
    memory: Memory,
    debug_rx: Receiver<DebugMessage>,
    monitor_tx: Sender<MonitorMessage>,
    io_tx: Sender<IoMessage>,
}

impl UiHost {
    pub(crate) fn new(
        debug_rx: Receiver<DebugMessage>,
        monitor_tx: Sender<MonitorMessage>,
        io_tx: Sender<IoMessage>,
    ) -> Self {
        Self {
            memory: Memory::default(),
            debug_rx,
            monitor_tx,
            io_tx,
        }
    }

    pub(crate) fn run(&self, image: Image) -> Result<()> {
        let monitor = Box::new(UiMonitor::new(self.monitor_tx.clone()));

        self.memory.store_image(&image)?;

        let mut cpu = Cpu::new(self.memory.view(), Some(monitor));
        cpu.reg.pc = image.start;

        let os = Os::emulate(OsEmulation::AcornStyle)?;
        os.load_into_vm(&mut cpu);

        let rti = MOS_6502
            .get_op_info(&Opcode::Rti)
            .ok_or_else(|| anyhow!("RTI must exist"))?
            .clone();

        let mut state = Stepping;
        loop {
            self.send_state(state);

            match state {
                Running => state = self.handle_running(&mut cpu, &os, &rti),
                Stepping => state = self.handle_stepping(&mut cpu, &os, &rti),
                Halted => state = self.handle_halted(&mut cpu),
                Stopped => break,
            }
        }

        Ok(())
    }

    fn send_state(&self, state: State) {
        _ = self.monitor_tx.send(MonitorMessage::NotifyState(state))
    }

    fn fetch_instruction(&self, cpu: &Cpu) {
        let instruction_info = InstructionInfo::fetch(cpu);
        _ = self.monitor_tx.send(MonitorMessage::BeforeExecute {
            total_cycles: cpu.total_cycles,
            reg: cpu.reg.clone(),
            instruction_info,
        });
    }

    fn handle_brk(&self, cpu: &mut Cpu, os: &Os, rti: &OpInfo, state: State) -> State {
        match os.is_os_vector(cpu) {
            Some(OSHALT) => Halted,
            Some(OSWRCH) => {
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

    fn handle_running(&self, cpu: &mut Cpu, os: &Os, rti: &OpInfo) -> State {
        loop {
            self.fetch_instruction(cpu);
            match self.debug_rx.try_recv() {
                Err(TryRecvError::Disconnected) => return Stopped,
                Err(TryRecvError::Empty) => {}
                Ok(_) => {}
            }

            if !cpu.step() {
                let new_state = self.handle_brk(cpu, os, rti, Running);
                if !matches!(new_state, Stepping) {
                    return new_state;
                }
            }
        }
    }

    fn handle_stepping(&self, cpu: &mut Cpu, os: &Os, rti: &OpInfo) -> State {
        loop {
            self.fetch_instruction(cpu);
            match self.debug_rx.recv() {
                Err(_) => return Stopped,
                Ok(m) => match m {
                    DebugMessage::Step => {}
                    DebugMessage::Run => return Running,
                    DebugMessage::Break => {}
                    DebugMessage::FetchMemory(address_range) => self.fetch_memory(address_range),
                    DebugMessage::SetPc(addr) => self.set_pc(cpu, addr),
                    DebugMessage::Go(addr) => {
                        p_set!(cpu.reg, B, false);
                        self.set_pc(cpu, addr);
                    }
                },
            }

            if !cpu.step() {
                let new_state = self.handle_brk(cpu, os, rti, Stepping);
                if !matches!(new_state, Stepping) {
                    return new_state;
                }
            }
        }
    }

    fn handle_halted(&self, cpu: &mut Cpu) -> State {
        self.fetch_instruction(cpu);
        loop {
            match self.debug_rx.recv() {
                Err(_) => return Stopped,
                Ok(m) => match m {
                    DebugMessage::Step => {}
                    DebugMessage::Run => {}
                    DebugMessage::Break => {}
                    DebugMessage::FetchMemory(address_range) => self.fetch_memory(address_range),
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

    fn fetch_memory(&self, address_range: AddressRange) {
        assert!(address_range.end >= address_range.begin);
        let snapshot = self.memory.snapshot(address_range.begin, address_range.end);
        _ = self.monitor_tx.send(MonitorMessage::FetchMemoryResponse {
            address_range,
            snapshot,
        });
    }

    fn set_pc(&self, cpu: &mut Cpu, addr: u16) {
        cpu.reg.pc = addr;
        self.fetch_instruction(cpu);
    }
}
