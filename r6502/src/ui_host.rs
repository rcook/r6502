use crate::{
    initialize_vm, AddressRange, DebugMessage, IoMessage, MonitorMessage, PollResult, Status,
    UiMonitor, VmStatus,
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
        let mut free_running = false;
        loop {
            while vm.step() {
                let result = self.poll(&mut vm, free_running);
                free_running = result.free_running;
                if !result.is_active {
                    // Handle disconnection
                    return Ok(VmStatus::Disconnected);
                }
            }

            match os.is_os_vector_brk(&vm) {
                Some(OSHALT) => {
                    self.monitor_tx
                        .send(MonitorMessage::Status(Status::Halted))
                        .expect("Must succeed");
                    return Ok(VmStatus::Halted);
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

    fn poll(&self, vm: &mut Vm, mut free_running: bool) -> PollResult {
        loop {
            if free_running {
                match self.debug_rx.try_recv() {
                    Err(TryRecvError::Disconnected) => {
                        return PollResult {
                            is_active: false,
                            free_running,
                        }
                    }
                    Err(TryRecvError::Empty) => {
                        return PollResult {
                            is_active: true,
                            free_running,
                        }
                    }
                    Ok(m) => match m {
                        DebugMessage::Step => {}
                        DebugMessage::Run => {}
                        DebugMessage::Break => free_running = false,
                        DebugMessage::FetchMemory(address_range) => {
                            self.fetch_memory(vm, address_range)
                        }
                        DebugMessage::SetPc(addr) => self.set_pc(vm, addr),
                    },
                }
            } else {
                match self.debug_rx.recv() {
                    Err(_) => {
                        return PollResult {
                            is_active: false,
                            free_running,
                        }
                    }
                    Ok(m) => match m {
                        DebugMessage::Step => {
                            return PollResult {
                                is_active: true,
                                free_running,
                            }
                        }
                        DebugMessage::Run => free_running = true,
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
        let instruction_info = InstructionInfo::fetch(vm);
        _ = self.monitor_tx.send(MonitorMessage::BeforeExecute {
            total_cycles: vm.total_cycles,
            reg: vm.s.reg.clone(),
            instruction_info,
        });
    }
}
