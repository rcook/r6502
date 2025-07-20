use crate::keyboard::{KeyCode, KeyModifiers};

#[derive(Debug)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}
