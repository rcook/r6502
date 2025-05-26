use crate::ControllerMessage;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct Thunk {
    tx: Sender<ControllerMessage>,
    step_count: Arc<Mutex<i32>>,
}

impl Thunk {
    pub(crate) fn new(tx: Sender<ControllerMessage>, step_count: Arc<Mutex<i32>>) -> Self {
        Self { tx, step_count }
    }

    pub(crate) fn step_count(&self) -> i32 {
        *self.step_count.lock().expect("Must succeed")
    }

    pub(crate) fn write_stdout(&self, c: char) {
        self.tx
            .send(ControllerMessage::WriteStdout(c))
            .expect("Must succeed")
    }

    pub(crate) fn println(&self, s: &str) {
        self.tx
            .send(ControllerMessage::Println(String::from(s)))
            .expect("Must succeed")
    }
}
