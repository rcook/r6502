use crate::{ControllerMessage, UIMessage};
use cursive::event::Key;
use cursive::view::Nameable;
use cursive::views::{LinearLayout, Panel, TextView};
use cursive::{Cursive, CursiveRunnable, CursiveRunner};
use std::sync::mpsc::{channel, Receiver, Sender};

pub(crate) struct UI {
    #[allow(unused)]
    controller_tx: Sender<ControllerMessage>,
    cursive: CursiveRunner<CursiveRunnable>,
    ui_tx: Sender<UIMessage>,
    ui_rx: Receiver<UIMessage>,
}

impl UI {
    pub(crate) fn new(controller_tx: Sender<ControllerMessage>) -> Self {
        let (ui_tx, ui_rx) = channel();
        let mut cursive = cursive::default().into_runner();

        cursive.add_layer(
            LinearLayout::vertical()
                .child(Panel::new(TextView::new("").with_name("stdout")))
                .child(TextView::new("").with_name("logger")),
        );

        cursive.add_global_callback(Key::Esc, Cursive::quit);
        cursive.set_fps(5);

        Self {
            controller_tx,
            cursive,
            ui_tx: ui_tx,
            ui_rx: ui_rx,
        }
    }

    pub(crate) fn tx(&self) -> &Sender<UIMessage> {
        &self.ui_tx
    }

    pub fn step(&mut self) -> bool {
        if !self.cursive.is_running() {
            return false;
        }

        while let Some(message) = self.ui_rx.try_iter().next() {
            match message {
                UIMessage::AppendStdoutChar(c) => {
                    self.cursive
                        .find_name::<TextView>("stdout")
                        .unwrap()
                        .append(c);
                }
                UIMessage::AppendLogLine(mut s) => {
                    s.push('\n');
                    self.cursive
                        .find_name::<TextView>("logger")
                        .unwrap()
                        .append(s);
                }
            }
        }

        self.cursive.step();
        true
    }
}
