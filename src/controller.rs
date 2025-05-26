use crate::{ControllerMessage, Thunk, UIMessage, UI};
use anyhow::Result;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};

pub(crate) struct Controller {
    rx: Receiver<ControllerMessage>,
    ui: UI,
    step_count: Arc<Mutex<i32>>,
    thunk: Thunk,
}

impl Controller {
    pub(crate) fn new() -> Result<Self> {
        let (tx, rx) = channel();
        let step_count = Arc::new(Mutex::new(0));
        Ok(Self {
            rx: rx,
            ui: UI::new(tx.clone())?,
            step_count: step_count.clone(),
            thunk: Thunk::new(tx, step_count),
        })
    }

    pub(crate) fn thunk(&self) -> &Thunk {
        &self.thunk
    }

    pub(crate) fn run(&mut self) {
        while self.ui.step() {
            while let Some(message) = self.rx.try_iter().next() {
                // Handle messages arriving from the UI.
                match message {
                    ControllerMessage::WriteStdout(c) => {
                        self.ui.tx().send(UIMessage::WriteStdout(c)).unwrap();
                    }
                    ControllerMessage::Println(s) => {
                        self.ui.tx().send(UIMessage::Println(s)).unwrap();
                    }
                    ControllerMessage::Step => {
                        *self.step_count.lock().expect("Must succeed") += 1;
                    }
                };
            }
        }
    }
}
