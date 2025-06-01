use crate::{
    initialize_vm, DebugMessage, IoMessage, MonitorMessage, PollResult, Status, UiMonitor, VmStatus,
};
use anyhow::Result;
use r6502lib::{Image, Memory, VmBuilder, OSHALT, OSWRCH};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

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
                let result = self.poll(&vm.s.memory, free_running);
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

    fn poll(&self, memory: &Memory, mut free_running: bool) -> PollResult {
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
                        DebugMessage::FetchMemory { begin, end } => {
                            self.fetch_memory(memory, begin, end)
                        }
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
                        DebugMessage::FetchMemory { begin, end } => {
                            self.fetch_memory(memory, begin, end)
                        }
                    },
                }
            }
        }
    }

    fn fetch_memory(&self, memory: &Memory, begin: u16, end: u16) {
        let begin_temp = begin as usize;
        let end_temp = end as usize;
        assert!(end_temp >= begin_temp && end_temp <= 0x10000);
        let count = end_temp - begin_temp + 1;
        let snapshot = memory.snapshot(begin_temp, begin_temp + count);
        _ = self.monitor_tx.send(MonitorMessage::FetchMemoryResponse {
            begin,
            end,
            snapshot,
        });
    }
}
