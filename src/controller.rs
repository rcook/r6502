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
        let mut cpu = Cpu::new(self.tx.clone());
        let cpu_tx = cpu.tx().clone();

        if let Some(program_info) = program_info {
            program_info.load(&mut cpu.memory)?;
            cpu.pc = program_info.start();
        }

        spawn(move || {
            run_vm(&mut cpu).unwrap();
        });

        while self.ui.step() {
            while let Some(message) = self.rx.try_iter().next() {
                match message {
                    ControllerMessage::WriteStdout(c) => self
                        .ui
                        .tx()
                        .send(UIMessage::WriteStdout(c))
                        .expect("Must succeed"),
                    ControllerMessage::Println(s) => self
                        .ui
                        .tx()
                        .send(UIMessage::Println(s))
                        .expect("Must succeed"),
                    ControllerMessage::ShowRegisters(s) => self
                        .ui
                        .tx()
                        .send(UIMessage::ShowRegisters(s))
                        .expect("Must succeed"),
                    ControllerMessage::Step => cpu_tx.send(CpuMessage::Step).expect("Must succeed"),
                    ControllerMessage::Run => cpu_tx.send(CpuMessage::Run).expect("Must succeed"),
                    ControllerMessage::Break => {
                        cpu_tx.send(CpuMessage::Break).expect("Must succeed")
                    }
                };
            }
        }

        Ok(())
    }
}
