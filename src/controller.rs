use crate::{run_vm, ControllerMessage, Cpu, CpuMessage, ProgramInfo, UIMessage, UI};
use anyhow::Result;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::spawn,
};

pub(crate) struct Controller {
    tx: Sender<ControllerMessage>,
    rx: Receiver<ControllerMessage>,
    ui: UI,
}

impl Controller {
    pub(crate) fn new() -> Result<Self> {
        let (tx, rx) = channel();
        let ui = UI::new(tx.clone())?;
        Ok(Self { tx, rx, ui })
    }

    pub(crate) fn run(&mut self, program_info: Option<ProgramInfo>) -> Result<()> {
        use crate::ControllerMessage::*;

        let mut cpu = Cpu::new(self.tx.clone());
        let cpu_tx = cpu.tx().clone();

        spawn(move || {
            run_vm(&mut cpu, program_info).unwrap();
        });

        while self.ui.step() {
            while let Some(message) = self.rx.try_iter().next() {
                match message {
                    WriteStdout(c) => self
                        .ui
                        .tx()
                        .send(UIMessage::WriteStdout(c))
                        .expect("Must succeed"),
                    Current(s) => self
                        .ui
                        .tx()
                        .send(UIMessage::Current(s))
                        .expect("Must succeed"),
                    History(s) => self
                        .ui
                        .tx()
                        .send(UIMessage::History(s))
                        .expect("Must succeed"),
                    Registers(s) => self
                        .ui
                        .tx()
                        .send(UIMessage::Registers(s))
                        .expect("Must succeed"),
                    Step => cpu_tx.send(CpuMessage::Step).expect("Must succeed"),
                    Run => cpu_tx.send(CpuMessage::Run).expect("Must succeed"),
                    Break => cpu_tx.send(CpuMessage::Break).expect("Must succeed"),
                };
            }
        }

        Ok(())
    }
}
