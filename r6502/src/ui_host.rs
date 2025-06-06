use crate::State::*;
use crate::{AddressRange, DebugMessage, IoMessage, MonitorMessage, State, UiMonitor};
use anyhow::{anyhow, Result};
use r6502lib::{
    p_set, Image, InstructionInfo, Memory, OpInfo, Opcode, Os, OsEmulation, Reg, Vm, VmState,
    MOS_6502, OSHALT, OSWRCH,
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
            memory: Memory::new(),
            debug_rx,
            monitor_tx,
            io_tx,
        }
    }

    pub(crate) fn run(&self, image: Image) -> Result<()> {
        let monitor = Box::new(UiMonitor::new(self.monitor_tx.clone()));

        self.memory.store_image(&image)?;

        let mut vm = Vm::new(monitor, VmState::new(Reg::default(), self.memory.view()));
        vm.s.reg.pc = image.start;

        let os = Os::emulate(OsEmulation::AcornStyle)?;
        os.load_into_vm(&mut vm);

        let rti = MOS_6502
            .get_op_info(&Opcode::Rti)
            .ok_or_else(|| anyhow!("RTI must exist"))?
            .clone();

        let mut state = Stepping;
        loop {
            self.send_state(state);

            match state {
                Running => state = self.handle_running(&mut vm, &os, &rti),
                Stepping => state = self.handle_stepping(&mut vm, &os, &rti),
                Halted => state = self.handle_halted(&mut vm),
                Stopped => break,
            }
        }

        Ok(())
    }

    fn send_state(&self, state: State) {
        _ = self.monitor_tx.send(MonitorMessage::NotifyState(state))
    }

    fn fetch_instruction(&self, vm: &Vm) {
        let instruction_info = InstructionInfo::fetch(vm);
        _ = self.monitor_tx.send(MonitorMessage::BeforeExecute {
            total_cycles: vm.total_cycles,
            reg: vm.s.reg.clone(),
            instruction_info,
        });
    }

    fn handle_brk(&self, vm: &mut Vm, os: &Os, rti: &OpInfo, state: State) -> State {
        match os.is_os_vector(vm) {
            Some(OSHALT) => Halted,
            Some(OSWRCH) => {
                self.io_tx
                    .send(IoMessage::WriteChar(vm.s.reg.a as char))
                    .expect("Must succeed");
                rti.execute_no_operand(&mut vm.s);
                state
            }
            _ => {
                _ = self.monitor_tx.send(MonitorMessage::NotifyInvalidBrk);
                Halted
            }
        }
    }

    fn handle_running(&self, vm: &mut Vm, os: &Os, rti: &OpInfo) -> State {
        loop {
            self.fetch_instruction(vm);
            match self.debug_rx.try_recv() {
                Err(TryRecvError::Disconnected) => return Stopped,
                Err(TryRecvError::Empty) => {}
                Ok(_) => {}
            }

            if !vm.step() {
                let new_state = self.handle_brk(vm, os, rti, Running);
                if !matches!(new_state, Stepping) {
                    return new_state;
                }
            }
        }
    }

    fn handle_stepping(&self, vm: &mut Vm, os: &Os, rti: &OpInfo) -> State {
        loop {
            self.fetch_instruction(vm);
            match self.debug_rx.recv() {
                Err(_) => return Stopped,
                Ok(m) => match m {
                    DebugMessage::Step => {}
                    DebugMessage::Run => return Running,
                    DebugMessage::Break => {}
                    DebugMessage::FetchMemory(address_range) => self.fetch_memory(address_range),
                    DebugMessage::SetPc(addr) => self.set_pc(vm, addr),
                    DebugMessage::Go(addr) => {
                        p_set!(vm.s.reg, B, false);
                        self.set_pc(vm, addr);
                    }
                },
            }

            if !vm.step() {
                let new_state = self.handle_brk(vm, os, rti, Stepping);
                if !matches!(new_state, Stepping) {
                    return new_state;
                }
            }
        }
    }

    fn handle_halted(&self, vm: &mut Vm) -> State {
        self.fetch_instruction(vm);
        loop {
            match self.debug_rx.recv() {
                Err(_) => return Stopped,
                Ok(m) => match m {
                    DebugMessage::Step => {}
                    DebugMessage::Run => {}
                    DebugMessage::Break => {}
                    DebugMessage::FetchMemory(address_range) => self.fetch_memory(address_range),
                    DebugMessage::SetPc(addr) => self.set_pc(vm, addr),
                    DebugMessage::Go(addr) => {
                        p_set!(vm.s.reg, B, false);
                        self.set_pc(vm, addr);
                        return Stepping;
                    }
                },
            }
        }
    }

    fn fetch_memory(&self, address_range: AddressRange) {
        let begin_temp = address_range.begin as usize;
        let end_temp = address_range.end as usize;
        assert!(end_temp >= begin_temp && end_temp <= 0x10000);
        let count = end_temp - begin_temp + 1;
        let snapshot = self.memory.snapshot(begin_temp, begin_temp + count);
        _ = self.monitor_tx.send(MonitorMessage::FetchMemoryResponse {
            address_range,
            snapshot,
        });
    }

    fn set_pc(&self, vm: &mut Vm, addr: u16) {
        vm.s.reg.pc = addr;
        self.fetch_instruction(vm);
    }
}
