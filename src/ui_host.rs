use crate::{
    Cycles, DebugMessage, Instruction, PollResult, RegisterFile, Status, StatusMessage, VMHost,
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
}

impl VMHost for UIHost {
    fn report_before_execute(&self, reg: &RegisterFile, cycles: Cycles, instruction: &Instruction) {
        self.status_tx
            .send(StatusMessage::BeforeExecute(
                reg.clone(),
                cycles,
                instruction.clone(),
            ))
            .expect("Must succeed")
    }

    fn poll(&self, mut free_running: bool) -> PollResult {
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
                    Ok(DebugMessage::Step) => {}
                    Ok(DebugMessage::Run) => {}
                    Ok(DebugMessage::Break) => free_running = false,
                }
            } else {
                match self.debug_rx.recv() {
                    Err(_) => {
                        return PollResult {
                            is_active: false,
                            free_running,
                        }
                    }
                    Ok(DebugMessage::Step) => {
                        return PollResult {
                            is_active: true,
                            free_running,
                        }
                    }
                    Ok(DebugMessage::Run) => free_running = true,
                    Ok(DebugMessage::Break) => {}
                }
            }
        }
    }

    fn report_after_execute(&self, reg: &RegisterFile, cycles: Cycles, instruction: &Instruction) {
        self.status_tx
            .send(StatusMessage::AfterExecute(
                reg.clone(),
                cycles,
                instruction.clone(),
            ))
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
