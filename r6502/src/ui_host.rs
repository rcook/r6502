use crate::{
    initialize_vm, AddressRange, DebugMessage, IoMessage, MonitorMessage, Status, UiMonitor,
    VmStatus,
};
use anyhow::Result;
use r6502lib::{Image, InstructionInfo, Vm, VmBuilder, OSHALT, OSWRCH};
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

    pub(crate) fn run(&self, image: Image) -> Result<VmStatus> {
        let monitor = Box::new(UiMonitor::new(self.monitor_tx.clone()));
        let mut vm = VmBuilder::default().monitor(monitor).build()?;
        let (os, rts) = initialize_vm(&mut vm, &image)?;

        self.fetch(&vm);

        let mut state = DebugState {
            disconnected: false,
            free_running: false,
            waiting: false,
        };

        loop {
            while state.waiting {
                self.poll(&mut vm, &mut state);
                if state.disconnected {
                    return Ok(VmStatus::Disconnected);
                }
            }

            loop {
                while vm.step() {
                    self.poll(&mut vm, &mut state);
                    if state.disconnected {
                        return Ok(VmStatus::Disconnected);
                    }
                }

                match os.is_os_vector_brk(&vm) {
                    Some(OSHALT) => {
                        self.monitor_tx
                            .send(MonitorMessage::Status(Status::Halted))
                            .expect("Must succeed");
                        state.free_running = false;
                        state.waiting = true;
                        break;
                    }
                    Some(OSWRCH) => {
                        self.io_tx
                            .send(IoMessage::WriteChar(vm.s.reg.a as char))
                            .expect("Must succeed");
                        os.return_from_os_vector_brk(&mut vm, &rts);
                    }
                    _ => todo!(),
                }
            }
        }
    }

    fn fetch(&self, vm: &Vm) {
        let instruction_info = InstructionInfo::fetch(vm);
        _ = self.monitor_tx.send(MonitorMessage::BeforeExecute {
            total_cycles: vm.total_cycles,
            reg: vm.s.reg.clone(),
            instruction_info,
        });
    }

    fn poll(&self, vm: &mut Vm, state: &mut DebugState) {
        loop {
            if state.free_running {
                match self.debug_rx.try_recv() {
                    Err(TryRecvError::Disconnected) => {
                        state.disconnected = true;
                        return;
                    }
                    Err(TryRecvError::Empty) => {
                        return;
                    }
                    Ok(m) => match m {
                        DebugMessage::Step => {}
                        DebugMessage::Run => {}
                        DebugMessage::Break => state.free_running = false,
                        DebugMessage::FetchMemory(address_range) => {
                            self.fetch_memory(vm, address_range)
                        }
                        DebugMessage::SetPc(addr) => {
                            self.set_pc(vm, addr);
                            state.waiting = true
                        }
                    },
                }
            } else {
                match self.debug_rx.recv() {
                    Err(_) => {
                        state.disconnected = true;
                        return;
                    }
                    Ok(m) => match m {
                        DebugMessage::Step => return,
                        DebugMessage::Run => state.free_running = true,
                        DebugMessage::Break => {}
                        DebugMessage::FetchMemory(address_range) => {
                            self.fetch_memory(vm, address_range)
                        }
                        DebugMessage::SetPc(addr) => self.set_pc(vm, addr),
                    },
                }
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
        self.fetch(vm);
    }
}

struct DebugState {
    disconnected: bool,
    free_running: bool,
    waiting: bool,
}
