use crate::{ControllerMessage, UIMessage};
use anyhow::Result;
use cursive::direction::Orientation;
use cursive::view::{Nameable, Scrollable};
use cursive::views::{LinearLayout, Menubar, NamedView, ScrollView, TextView};
use cursive::{Cursive, CursiveRunnable, CursiveRunner};
use cursive_multiplex::Mux;
use std::sync::mpsc::{channel, Receiver, Sender};

pub(crate) struct UI {
    #[allow(unused)]
    controller_tx: Sender<ControllerMessage>,
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
        let logger =
            mux.add_right_of(TextView::new("").scrollable().with_name("logger"), node_id)?;

        let stdout =
            mux.add_right_of(TextView::new("").with_name("stdout").scrollable(), logger)?;

        mux.set_container_split_ratio(stdout, 0.7).unwrap();

        let mut linear = LinearLayout::new(Orientation::Vertical);
        let mux_later = NamedView::new("Mux", mux);
        linear.add_child(mux_later);
        let mut menu_bar = Menubar::new();
        menu_bar.add_leaf("Quit", Cursive::quit);
        linear.add_child(menu_bar);

        cursive.add_fullscreen_layer(linear);
        cursive.add_global_callback('q', Cursive::quit);

        cursive.set_fps(30);

        Ok(Self {
            controller_tx,
            cursive,
            tx,
            rx,
        })
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
                UIMessage::AppendStdoutChar(c) => {
                    self.cursive
                        .find_name::<TextView>("stdout")
                        .unwrap()
                        .append(c);
                }
                UIMessage::AppendLogLine(mut s) => {
                    s.push('\n');
                    let mut logger = self
                        .cursive
                        .find_name::<ScrollView<TextView>>("logger")
                        .expect("Must exist");
                    logger.get_inner_mut().append(s);

                    // TBD: Figure out why this doesn't work!
                    logger.scroll_to_bottom();
                }
            }
        }

        self.cursive.step();
        true
    }
}
