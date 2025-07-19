use crate::events::{KeyCode, KeyEventKind, KeyModifiers};

#[derive(Debug)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    pub kind: KeyEventKind,
}
