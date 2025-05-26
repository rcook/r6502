use crate::{ControllerMessage, UIMessage, UI};
use anyhow::Result;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub(crate) struct Controller {
    tx: Sender<ControllerMessage>,
    rx: Receiver<ControllerMessage>,
    ui: UI,
}

impl Controller {
    pub(crate) fn new() -> Result<Self> {
        let (tx, rx) = channel();
        Ok(Self {
            tx: tx.clone(),
            rx: rx,
            ui: UI::new(tx),
        })
    }

    pub fn run(&mut self) {
        while self.ui.step() {
            while let Some(message) = self.rx.try_iter().next() {
                // Handle messages arriving from the UI.
                match message {
                    ControllerMessage::AppendStdoutChar(c) => {
                        self.ui.tx().send(UIMessage::AppendStdoutChar(c)).unwrap();
                    }
                    ControllerMessage::AppendLogLine(s) => {
                        self.ui.tx().send(UIMessage::AppendLogLine(s)).unwrap();
                    }
                };
            }
        }
    }

    pub fn tx(&self) -> &Sender<ControllerMessage> {
        &self.tx
    }
}
