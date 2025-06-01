use crate::{DebugMessage, MonitorMessage, PollResult};
use r6502lib::Memory;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub(crate) struct UiHost {
    debug_rx: Receiver<DebugMessage>,
    monitor_tx: Sender<MonitorMessage>,
}

impl UiHost {
    pub(crate) fn new(
        debug_rx: Receiver<DebugMessage>,
        monitor_tx: Sender<MonitorMessage>,
    ) -> Self {
        Self {
            debug_rx,
            monitor_tx,
        }
    }

    pub(crate) fn poll(&self, memory: &Memory, mut free_running: bool) -> PollResult {
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
