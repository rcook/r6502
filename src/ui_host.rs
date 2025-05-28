use crate::{
    Cycles, DebugMessage, Instruction, MachineState, PollResult, RegisterFile, Status,
    StatusMessage, VMHost,
};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub(crate) struct UIHost {
    debug_rx: Receiver<DebugMessage>,
    status_tx: Sender<StatusMessage>,
}

impl UIHost {
    pub(crate) fn new(debug_rx: Receiver<DebugMessage>, status_tx: Sender<StatusMessage>) -> Self {
        Self {
            debug_rx,
            status_tx,
        }
    }

    fn fetch_memory(&self, memory: &[u8], begin: u16, end: u16) {
        let begin_temp = begin as usize;
        let end_temp = end as usize;
        assert!(end_temp >= begin_temp && end_temp <= 0x10000);
        let count = end_temp - begin_temp + 1;
        let snapshot = memory[begin_temp..begin_temp + count].to_vec();
        _ = self.status_tx.send(StatusMessage::FetchMemoryResponse {
            begin,
            end,
            snapshot,
        });
    }
}

impl VMHost for UIHost {
    fn report_before_execute(&self, reg: &RegisterFile, cycles: Cycles, instruction: &Instruction) {
        self.status_tx
            .send(StatusMessage::BeforeExecute {
                reg: reg.clone(),
                cycles,
                instruction: instruction.clone(),
            })
            .expect("Must succeed")
    }

    fn poll(&self, machine_state: &MachineState, mut free_running: bool) -> PollResult {
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
                            self.fetch_memory(&machine_state.memory, begin, end)
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
                            self.fetch_memory(&machine_state.memory, begin, end)
                        }
                    },
                }
            }
        }
    }

    fn report_after_execute(&self, reg: &RegisterFile, cycles: Cycles, instruction: &Instruction) {
        self.status_tx
            .send(StatusMessage::AfterExecute {
                reg: reg.clone(),
                cycles,
                instruction: instruction.clone(),
            })
            .expect("Must succeed")
    }

    fn report_status(&self, status: Status) {
        self.status_tx
            .send(StatusMessage::Status(status))
            .expect("Must succeed")
    }

    fn write_stdout(&self, c: char) {
        self.status_tx
            .send(StatusMessage::WriteStdout(c))
            .expect("Must succeed")
    }
}
