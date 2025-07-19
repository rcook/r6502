use crate::emulator::KeyEvent;

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
}
