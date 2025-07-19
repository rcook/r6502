use crate::events::KeyEvent;

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
}
