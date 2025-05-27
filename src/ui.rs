use crate::{ControllerMessage, UIMessage};
use anyhow::Result;
use cursive::direction::Orientation;
use cursive::view::{Nameable, Resizable, ScrollStrategy, Scrollable};
use cursive::views::{LinearLayout, Menubar, NamedView, TextView};
use cursive::{Cursive, CursiveRunnable, CursiveRunner};
use cursive_multiplex::Mux;
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

        let mut mux = Mux::new();
        let root_id = mux.root().build().expect("Must have ID");

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

        let current_id = mux.add_right_of(current, root_id)?;
        let history_id = mux.add_below(history, current_id)?;
        let stdout_id = mux.add_right_of(stdout, history_id)?;
        let help_id = mux.add_below(help, stdout_id)?;
        _ = mux.add_below(registers, help_id);

        let mut linear = LinearLayout::new(Orientation::Vertical);
        let mux_layer = NamedView::new("Mux", mux);
        linear.add_child(mux_layer);
        let mut menu_bar = Menubar::new();
        menu_bar.add_leaf("Quit", Cursive::quit);
        linear.add_child(menu_bar);

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
