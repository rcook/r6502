use crate::{ControllerMessage, UIMessage};
use anyhow::Result;
use cursive::align::HAlign;
use cursive::direction::Orientation;
use cursive::view::{Nameable, Resizable, ScrollStrategy, Scrollable};
use cursive::views::{LinearLayout, Panel, TextView};
use cursive::{Cursive, CursiveRunnable, CursiveRunner};
use std::sync::mpsc::{channel, Receiver, Sender};

const CURRENT_NAME: &str = "current";
const DISASSEMBLY_NAME: &str = "disassembly";
const STDOUT_NAME: &str = "stdout";
const REGISTERS_NAME: &str = "registers";
const CYCLES_NAME: &str = "cycles";

pub(crate) struct UI {
    cursive: CursiveRunner<CursiveRunnable>,
    tx: Sender<UIMessage>,
    rx: Receiver<UIMessage>,
}

impl UI {
    pub(crate) fn new(controller_tx: Sender<ControllerMessage>) -> Result<Self> {
        let mut cursive = Self::make_ui();
        Self::add_callbacks(&mut cursive, controller_tx);
        let (tx, rx) = channel();
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
                Disassembly(mut s) => {
                    s.push('\n');
                    self.cursive
                        .find_name::<TextView>(DISASSEMBLY_NAME)
                        .expect("Must exist")
                        .append(s);
                }
                Registers(s) => {
                    self.cursive
                        .find_name::<TextView>(REGISTERS_NAME)
                        .expect("Must exist")
                        .set_content(s);
                }
                Cycles(s) => {
                    self.cursive
                        .find_name::<TextView>(CYCLES_NAME)
                        .expect("Must exist")
                        .set_content(s);
                }
            }
        }

        self.cursive.step();
        true
    }

    fn make_ui() -> CursiveRunner<CursiveRunnable> {
        fn panel<V>(view: V, label: &str) -> Panel<V> {
            Panel::new(view).title(label).title_position(HAlign::Left)
        }

        let current = TextView::new("").with_name(CURRENT_NAME).fixed_height(1);
        let registers = TextView::new("").with_name(REGISTERS_NAME);
        let cycles = TextView::new("").with_name(CYCLES_NAME);
        let disassembly = TextView::new("")
            .with_name(DISASSEMBLY_NAME)
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let stdout = TextView::new("")
            .with_name(STDOUT_NAME)
            .full_width()
            .full_height()
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
        let help = TextView::new("Q: Quit\nSpace: Step\nR: Run\nB: Break")
            .scrollable()
            .scroll_strategy(ScrollStrategy::KeepRow);

        let execution = LinearLayout::new(Orientation::Vertical)
            .child(panel(current, "PC"))
            .child(panel(registers, "Registers"))
            .child(panel(cycles, "Cycles"))
            .child(panel(disassembly, "Disassembly"));

        let info = LinearLayout::new(Orientation::Vertical)
            .child(panel(stdout, "Output"))
            .child(panel(help, "Help"));

        let dashboard = LinearLayout::new(Orientation::Horizontal)
            .child(execution)
            .child(info);

        let mut cursive = cursive::default().into_runner();
        cursive.add_fullscreen_layer(dashboard);

        cursive.set_fps(30);
        cursive
    }

    fn add_callbacks(
        cursive: &mut CursiveRunner<CursiveRunnable>,
        controller_tx: Sender<ControllerMessage>,
    ) {
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
    }
}
