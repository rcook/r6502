use crate::{ControllerMessage, Thunk, UIMessage, VMMessage, UI};
use anyhow::Result;
use std::sync::mpsc::{channel, Receiver, Sender};

pub(crate) struct Controller {
    vm_tx: Sender<VMMessage>,
    rx: Receiver<ControllerMessage>,
    ui: UI,
    thunk: Thunk,
}

impl Controller {
    pub(crate) fn new(vm_tx: Sender<VMMessage>) -> Result<Self> {
        let (tx, rx) = channel();
        Ok(Self {
            vm_tx,
            rx: rx,
            ui: UI::new(tx.clone())?,
            thunk: Thunk::new(tx),
        })
    }

    pub(crate) fn thunk(&self) -> Thunk {
        self.thunk.clone()
    }

    pub(crate) fn run(&mut self) {
        while self.ui.step() {
            while let Some(message) = self.rx.try_iter().next() {
                match message {
                    ControllerMessage::WriteStdout(c) => {
                        self.ui
                            .tx()
                            .send(UIMessage::WriteStdout(c))
                            .expect("Must succeed");
                    }
                    ControllerMessage::Println(s) => {
                        self.ui
                            .tx()
                            .send(UIMessage::Println(s))
                            .expect("Must succeed");
                    }
                    ControllerMessage::Step => {
                        self.vm_tx.send(VMMessage::Step).expect("Must succeed");
                    }
                    ControllerMessage::Run => {
                        self.vm_tx.send(VMMessage::Run).expect("Must succeed");
                    }
                    ControllerMessage::Break => {
                        self.vm_tx.send(VMMessage::Break).expect("Must succeed");
                    }
                };
            }
        }
    }
}
