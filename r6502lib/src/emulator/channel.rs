use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Channel<T> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Channel<T> {
    #[must_use]
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self { tx, rx }
    }
}
