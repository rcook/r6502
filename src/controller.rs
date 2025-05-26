use crate::{ControllerMessage, CpuMessage, UIMessage, UI};
use anyhow::Result;
use std::sync::mpsc::{channel, Receiver, Sender};

pub(crate) struct Controller {
    vm_tx: Sender<CpuMessage>,
    tx: Sender<ControllerMessage>,
    rx: Receiver<ControllerMessage>,
    ui: UI,
}

impl Controller {
    pub(crate) fn new(vm_tx: Sender<CpuMessage>) -> Result<Self> {
        let (tx, rx) = channel();
        let ui = UI::new(tx.clone())?;
        Ok(Self { vm_tx, tx, rx, ui })
    }

    pub(crate) fn tx(&self) -> &Sender<ControllerMessage> {
        &self.tx
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
                        self.vm_tx.send(CpuMessage::Step).expect("Must succeed");
                    }
                    ControllerMessage::Run => {
                        self.vm_tx.send(CpuMessage::Run).expect("Must succeed");
                    }
                    ControllerMessage::Break => {
                        self.vm_tx.send(CpuMessage::Break).expect("Must succeed");
                    }
                };
            }
        }
    }
}
