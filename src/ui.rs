use crate::{ControllerMessage, UIMessage};
use anyhow::Result;
use cursive::direction::Orientation;
use cursive::view::{Nameable, Resizable, ScrollStrategy, Scrollable};
use cursive::views::{LinearLayout, Menubar, NamedView, TextView};
use cursive::{Cursive, CursiveRunnable, CursiveRunner};
use cursive_multiplex::Mux;
use std::sync::mpsc::{channel, Receiver, Sender};

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

        let node_id = mux.root().build().expect("Must have ID");
        let logger = mux.add_right_of(
            TextView::new("")
                .with_name("logger")
                .full_width()
                .full_height()
                .scrollable()
                .scroll_strategy(ScrollStrategy::StickToBottom),
            node_id,
        )?;

        let stdout = mux.add_right_of(
            TextView::new("")
                .with_name("stdout")
                .full_width()
                .full_height()
                .scrollable()
                .scroll_strategy(ScrollStrategy::StickToBottom),
            logger,
        )?;

        _ = mux.add_below(
            TextView::new("Q: Quit\nSpace: Step\nR: Run\nB: Break"),
            stdout,
        )?;

        mux.set_container_split_ratio(stdout, 0.7).unwrap();

        let mut linear = LinearLayout::new(Orientation::Vertical);
        let mux_later = NamedView::new("Mux", mux);
        linear.add_child(mux_later);
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
        if !self.cursive.is_running() {
            return false;
        }

        while let Some(message) = self.rx.try_iter().next() {
            match message {
                UIMessage::WriteStdout(c) => {
                    self.cursive
                        .find_name::<TextView>("stdout")
                        .unwrap()
                        .append(c);
                }
                UIMessage::Println(mut s) => {
                    s.push('\n');
                    self.cursive
                        .find_name::<TextView>("logger")
                        .expect("Must exist")
                        .append(s);
                }
            }
        }

        self.cursive.step();
        true
    }
}
