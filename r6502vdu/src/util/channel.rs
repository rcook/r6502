use std::sync::mpsc::{Receiver, Sender, channel};

pub struct Channel<T> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self { tx, rx }
    }
}
