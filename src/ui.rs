use crate::{ControllerMessage, UIMessage};
use anyhow::Result;
use cursive::direction::Orientation;
use cursive::view::{Nameable, Resizable, ScrollStrategy, Scrollable};
use cursive::views::{LinearLayout, TextView};
use cursive::{Cursive, CursiveRunnable, CursiveRunner};
use std::sync::mpsc::{channel, Receiver, Sender};

const CURRENT_NAME: &str = "current";
const HISTORY_NAME: &str = "history";
const STDOUT_NAME: &str = "stdout";
const REGISTERS_NAME: &str = "registers";

pub(crate) struct UI {
    cursive: CursiveRunner<CursiveRunnable>,
    tx: Sender<UIMessage>,
    rx: Receiver<UIMessage>,
}

impl UI {
    pub(crate) fn new(controller_tx: Sender<ControllerMessage>) -> Result<Self> {
        let (tx, rx) = channel();
        let mut cursive = cursive::default().into_runner();

        let current = TextView::new("")
            .with_name(CURRENT_NAME)
            .full_width()
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let history = TextView::new("")
            .with_name(HISTORY_NAME)
            .full_width()
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let stdout = TextView::new("")
            .with_name(STDOUT_NAME)
            .full_width()
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let help = TextView::new("Q: Quit\nSpace: Step\nR: Run\nB: Break");
        let registers = TextView::new("").with_name(REGISTERS_NAME);

        let linear = LinearLayout::new(Orientation::Vertical)
            .child(current)
            .child(history)
            .child(stdout)
            .child(help)
            .child(registers);

        cursive.add_fullscreen_layer(linear);
        cursive.add_global_callback('q', Cursive::quit);
        let temp = controller_tx.clone();
        cursive.add_global_callback(' ', move |_| {
            temp.send(ControllerMessage::Step).expect("Must succeed")
        });
        let temp = controller_tx.clone();
        cursive.add_global_callback('r', move |_| {
            temp.send(ControllerMessage::Run).expect("Must succeed")
        });
        let temp = controller_tx.clone();
        cursive.add_global_callback('b', move |_| {
            temp.send(ControllerMessage::Break).expect("Must succeed")
        });

        cursive.set_fps(30);

        Ok(Self { cursive, tx, rx })
    }

    pub(crate) fn tx(&self) -> &Sender<UIMessage> {
        &self.tx
    }

    pub fn step(&mut self) -> bool {
        use crate::UIMessage::*;

        if !self.cursive.is_running() {
            return false;
        }

        while let Some(message) = self.rx.try_iter().next() {
            match message {
                WriteStdout(c) => {
                    self.cursive
                        .find_name::<TextView>(STDOUT_NAME)
                        .unwrap()
                        .append(c);
                }
                Current(s) => {
                    self.cursive
                        .find_name::<TextView>(CURRENT_NAME)
                        .expect("Must exist")
                        .set_content(s);
                }
                History(mut s) => {
                    s.push('\n');
                    self.cursive
                        .find_name::<TextView>(HISTORY_NAME)
                        .expect("Must exist")
                        .append(s);
                }
                Registers(s) => {
                    self.cursive
                        .find_name::<TextView>(REGISTERS_NAME)
                        .expect("Must exist")
                        .set_content(s);
                }
            }
        }

        self.cursive.step();
        true
    }
}
