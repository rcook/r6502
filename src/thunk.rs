use crate::ControllerMessage;
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub(crate) struct Thunk {
    tx: Sender<ControllerMessage>,
}

impl Thunk {
    pub(crate) fn new(tx: Sender<ControllerMessage>) -> Self {
        Self { tx }
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
