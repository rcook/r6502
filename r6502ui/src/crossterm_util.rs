use cursive::backends::crossterm::crossterm::event::{
    Event as Event_crossterm, KeyCode as KeyCode_crossterm, KeyEvent as KeyEvent_crossterm,
    KeyEventKind as KeyEventKind_crossterm, KeyModifiers as KeyModifiers_crossterm,
};
use r6502core::events::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[must_use]
pub fn translate_event(event: &Event_crossterm) -> Option<Event> {
    match event {
        Event_crossterm::Key(key_event) => Some(Event::Key(translate_key_event(key_event)?)),
        _ => None,
    }
}

#[must_use]
pub fn translate_key_event(key: &KeyEvent_crossterm) -> Option<KeyEvent> {
    Some(KeyEvent {
        code: translate_key_code(key.code)?,
        modifiers: translate_key_modifiers(key.modifiers)?,
        kind: translate_key_event_kind(key.kind),
    })
}

#[must_use]
pub fn translate_key_code(key_code: KeyCode_crossterm) -> Option<KeyCode> {
    match key_code {
        KeyCode_crossterm::Backspace => Some(KeyCode::Backspace),
        KeyCode_crossterm::Enter => Some(KeyCode::Enter),
        KeyCode_crossterm::Left => Some(KeyCode::Left),
        KeyCode_crossterm::Right => Some(KeyCode::Right),
        KeyCode_crossterm::Up => Some(KeyCode::Up),
        KeyCode_crossterm::Down => Some(KeyCode::Down),
        KeyCode_crossterm::Home => Some(KeyCode::Home),
        KeyCode_crossterm::End => Some(KeyCode::End),
        KeyCode_crossterm::PageUp => Some(KeyCode::PageUp),
        KeyCode_crossterm::PageDown => Some(KeyCode::PageDown),
        KeyCode_crossterm::Tab => Some(KeyCode::Tab),
        KeyCode_crossterm::BackTab => Some(KeyCode::BackTab),
        KeyCode_crossterm::Delete => Some(KeyCode::Delete),
        KeyCode_crossterm::Insert => Some(KeyCode::Insert),
        KeyCode_crossterm::F(n) => Some(KeyCode::F(n)),
        KeyCode_crossterm::Char(c) => Some(KeyCode::Char(c)),
        KeyCode_crossterm::Null => Some(KeyCode::Null),
        KeyCode_crossterm::Esc => Some(KeyCode::Esc),
        KeyCode_crossterm::CapsLock => Some(KeyCode::CapsLock),
        KeyCode_crossterm::ScrollLock => Some(KeyCode::ScrollLock),
        KeyCode_crossterm::NumLock => Some(KeyCode::NumLock),
        KeyCode_crossterm::PrintScreen => Some(KeyCode::PrintScreen),
        KeyCode_crossterm::Pause => Some(KeyCode::Pause),
        KeyCode_crossterm::Menu => Some(KeyCode::Menu),
        KeyCode_crossterm::KeypadBegin => Some(KeyCode::KeypadBegin),
        KeyCode_crossterm::Media(_) | KeyCode_crossterm::Modifier(_) => None,
    }
}

#[must_use]
pub fn translate_key_modifiers(key_modifiers: KeyModifiers_crossterm) -> Option<KeyModifiers> {
    match key_modifiers {
        KeyModifiers_crossterm::SHIFT => Some(KeyModifiers::SHIFT),
        KeyModifiers_crossterm::CONTROL => Some(KeyModifiers::CONTROL),
        KeyModifiers_crossterm::ALT => Some(KeyModifiers::ALT),
        KeyModifiers_crossterm::SUPER => Some(KeyModifiers::SUPER),
        KeyModifiers_crossterm::HYPER => Some(KeyModifiers::HYPER),
        KeyModifiers_crossterm::META => Some(KeyModifiers::META),
        KeyModifiers_crossterm::NONE => Some(KeyModifiers::NONE),
        _ => None,
    }
}

#[must_use]
pub fn translate_key_event_kind(key_event_kind: KeyEventKind_crossterm) -> KeyEventKind {
    match key_event_kind {
        KeyEventKind_crossterm::Press => KeyEventKind::Press,
        KeyEventKind_crossterm::Repeat => KeyEventKind::Repeat,
        KeyEventKind_crossterm::Release => KeyEventKind::Release,
    }
}
