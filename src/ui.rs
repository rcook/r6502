use crate::{CpuMessage, Status as _Status, UIMessage};
use anyhow::Result;
use cursive::align::HAlign;
use cursive::direction::Orientation;
use cursive::view::{Nameable, Resizable, ScrollStrategy, Scrollable};
use cursive::views::{LinearLayout, Panel, TextView};
use cursive::{Cursive, CursiveRunnable, CursiveRunner};
use std::sync::mpsc::{Receiver, Sender};

const CURRENT_NAME: &str = "current";
const DISASSEMBLY_NAME: &str = "disassembly";
const STATUS_NAME: &str = "status";
const STDOUT_NAME: &str = "stdout";
const REGISTERS_NAME: &str = "registers";
const CYCLES_NAME: &str = "cycles";

pub(crate) struct UI {
    cursive: CursiveRunner<CursiveRunnable>,
    ui_rx: Receiver<UIMessage>,
}

impl UI {
    pub(crate) fn new(ui_rx: Receiver<UIMessage>, cpu_tx: Sender<CpuMessage>) -> Result<Self> {
        let mut cursive = Self::make_ui();
        Self::add_callbacks(&mut cursive, cpu_tx);
        Ok(Self { cursive, ui_rx })
    }

    pub fn step(&mut self) -> bool {
        use crate::UIMessage::*;

        if !self.cursive.is_running() {
            return false;
        }

        while let Some(message) = self.ui_rx.try_iter().next() {
            match message {
                BeforeExecute(reg, cycles, instruction) => {
                    self.cursive
                        .find_name::<TextView>(REGISTERS_NAME)
                        .expect("Must exist")
                        .set_content(reg.pretty());
                    self.cursive
                        .find_name::<TextView>(CYCLES_NAME)
                        .expect("Must exist")
                        .set_content(format!("cycles={cycles}"));
                    self.cursive
                        .find_name::<TextView>(CURRENT_NAME)
                        .expect("Must exist")
                        .set_content(instruction.pretty_current());
                }
                AfterExecute(reg, cycles, instruction) => {
                    self.cursive
                        .find_name::<TextView>(REGISTERS_NAME)
                        .expect("Must exist")
                        .set_content(reg.pretty());
                    self.cursive
                        .find_name::<TextView>(CYCLES_NAME)
                        .expect("Must exist")
                        .set_content(format!("cycles={cycles}"));

                    let mut s = instruction.pretty_disassembly();
                    s.push('\n');
                    self.cursive
                        .find_name::<TextView>(DISASSEMBLY_NAME)
                        .expect("Must exist")
                        .append(s);
                }
                Status(status) => {
                    let mut s = match status {
                        _Status::Halted => String::from("Halted"),
                    };
                    s.push('\n');
                    self.cursive
                        .find_name::<TextView>(STATUS_NAME)
                        .expect("Must exist")
                        .append(s);
                }
                WriteStdout(c) => {
                    self.cursive
                        .find_name::<TextView>(STDOUT_NAME)
                        .expect("Must exist")
                        .append(c);
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

        let status = TextView::new("")
            .with_name(STATUS_NAME)
            .fixed_height(3)
            .scrollable()
            .scroll_strategy(ScrollStrategy::StickToBottom);
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
            .child(panel(disassembly, "Disassembly"))
            .child(panel(status, "Status"));

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

    fn add_callbacks(cursive: &mut CursiveRunner<CursiveRunnable>, cpu_tx: Sender<CpuMessage>) {
        use crate::CpuMessage::*;

        cursive.add_global_callback('q', Cursive::quit);
        let tx = cpu_tx.clone();
        cursive.add_global_callback(' ', move |_| _ = tx.send(Step));
        let tx = cpu_tx.clone();
        cursive.add_global_callback('r', move |_| _ = tx.send(Run));
        let tx = cpu_tx.clone();
        cursive.add_global_callback('b', move |_| _ = tx.send(Break));
    }
}
