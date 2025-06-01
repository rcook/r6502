#![allow(unused)]

use crate::State::*;
use crate::{
    initialize_vm, AddressRange, DebugMessage, IoMessage, MonitorMessage, State, UiMonitor,
};
use anyhow::Result;
use r6502lib::{p_set, Image, InstructionInfo, OpInfo, Os, Vm, VmBuilder, OSHALT, OSWRCH};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

// TBD: Come up with a better name for this struct!
pub(crate) struct UiHost {
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
            debug_rx,
            monitor_tx,
            io_tx,
        }
    }

    pub(crate) fn run(&self, image: Image) -> Result<()> {
        let monitor = Box::new(UiMonitor::new(self.monitor_tx.clone()));
        let mut vm = VmBuilder::default().monitor(monitor).build()?;
        let (os, rts) = initialize_vm(&mut vm, &image)?;

        let mut state = Stepping;
        loop {
            self.send_state(state);

            match state {
                Running => state = self.handle_running(&mut vm, &os, &rts),
                Stepping => state = self.handle_stepping(&mut vm, &os, &rts),
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

    fn handle_brk(&self, vm: &mut Vm, os: &Os, rts: &OpInfo, state: State) -> State {
        match os.is_os_vector_brk(&vm) {
            Some(OSHALT) => Halted,
            Some(OSWRCH) => {
                self.io_tx
                    .send(IoMessage::WriteChar(vm.s.reg.a as char))
                    .expect("Must succeed");
                os.return_from_os_vector_brk(vm, &rts);
                state
            }
            _ => {
                _ = self.monitor_tx.send(MonitorMessage::NotifyInvalidBrk);
                Halted
            }
        }
    }

    fn handle_running(&self, vm: &mut Vm, os: &Os, rts: &OpInfo) -> State {
        loop {
            self.fetch_instruction(vm);
            match self.debug_rx.try_recv() {
                Err(TryRecvError::Disconnected) => return Stopped,
                Err(TryRecvError::Empty) => {}
                Ok(_) => {}
            }

            if !vm.step() {
                let new_state = self.handle_brk(vm, os, rts, Running);
                if !matches!(new_state, Stepping) {
                    return new_state;
                }
            }
        }
    }

    fn handle_stepping(&self, vm: &mut Vm, os: &Os, rts: &OpInfo) -> State {
        loop {
            self.fetch_instruction(vm);
            match self.debug_rx.recv() {
                Err(_) => return Stopped,
                Ok(m) => match m {
                    DebugMessage::Step => {}
                    DebugMessage::Run => return Running,
                    DebugMessage::Break => {}
                    DebugMessage::FetchMemory(address_range) => {
                        self.fetch_memory(vm, address_range)
                    }
                    DebugMessage::SetPc(addr) => self.set_pc(vm, addr),
                    DebugMessage::Go(addr) => {
                        p_set!(vm.s.reg, B, false);
                        self.set_pc(vm, addr);
                    }
                },
            }

            if !vm.step() {
                let new_state = self.handle_brk(vm, os, rts, Stepping);
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
                    DebugMessage::FetchMemory(address_range) => {
                        self.fetch_memory(vm, address_range)
                    }
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

    fn fetch_memory(&self, vm: &Vm, address_range: AddressRange) {
        let begin_temp = address_range.begin as usize;
        let end_temp = address_range.end as usize;
        assert!(end_temp >= begin_temp && end_temp <= 0x10000);
        let count = end_temp - begin_temp + 1;
        let snapshot = vm.s.memory.snapshot(begin_temp, begin_temp + count);
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

struct DebugState {
    disconnected: bool,
    free_running: bool,
    waiting: bool,
}
