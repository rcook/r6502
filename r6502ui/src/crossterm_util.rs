use cursive::backends::crossterm::crossterm::event::{
    Event as Event_crossterm, KeyCode as KeyCode_crossterm, KeyEvent as KeyEvent_crossterm,
    KeyEventKind as KeyEventKind_crossterm, KeyModifiers as KeyModifiers_crossterm,
};
use r6502lib::emulator::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[must_use]
pub fn translate_event(event: &Event_crossterm) -> Event {
    match event {
        Event_crossterm::Key(key_event) => Event::Key(translate_key_event(key_event)),
        _ => unimplemented!(),
    }
}

#[must_use]
pub fn translate_key_event(key: &KeyEvent_crossterm) -> KeyEvent {
    KeyEvent {
        code: translate_key_code(key.code),
        modifiers: translate_key_modifiers(key.modifiers),
        kind: translate_key_event_kind(key.kind),
    }
}

#[must_use]
pub fn translate_key_code(key_code: KeyCode_crossterm) -> KeyCode {
    match key_code {
        KeyCode_crossterm::Backspace => KeyCode::Backspace,
        KeyCode_crossterm::Enter => KeyCode::Enter,
        KeyCode_crossterm::Left => KeyCode::Left,
        KeyCode_crossterm::Right => KeyCode::Right,
        KeyCode_crossterm::Up => KeyCode::Up,
        KeyCode_crossterm::Down => KeyCode::Down,
        KeyCode_crossterm::Home => KeyCode::Home,
        KeyCode_crossterm::End => KeyCode::End,
        KeyCode_crossterm::PageUp => KeyCode::PageUp,
        KeyCode_crossterm::PageDown => KeyCode::PageDown,
        KeyCode_crossterm::Tab => KeyCode::Tab,
        KeyCode_crossterm::BackTab => KeyCode::BackTab,
        KeyCode_crossterm::Delete => KeyCode::Delete,
        KeyCode_crossterm::Insert => KeyCode::Insert,
        KeyCode_crossterm::F(n) => KeyCode::F(n),
        KeyCode_crossterm::Char(c) => KeyCode::Char(c),
        KeyCode_crossterm::Null => KeyCode::Null,
        KeyCode_crossterm::Esc => KeyCode::Esc,
        KeyCode_crossterm::CapsLock => KeyCode::CapsLock,
        KeyCode_crossterm::ScrollLock => KeyCode::ScrollLock,
        KeyCode_crossterm::NumLock => KeyCode::NumLock,
        KeyCode_crossterm::PrintScreen => KeyCode::PrintScreen,
        KeyCode_crossterm::Pause => KeyCode::Pause,
        KeyCode_crossterm::Menu => KeyCode::Menu,
        KeyCode_crossterm::KeypadBegin => KeyCode::KeypadBegin,
        KeyCode_crossterm::Media(_media_key_code) => unimplemented!(),
        KeyCode_crossterm::Modifier(_modifier_key_code) => unimplemented!(),
    }
}

#[must_use]
pub fn translate_key_modifiers(key_modifiers: KeyModifiers_crossterm) -> KeyModifiers {
    match key_modifiers {
        KeyModifiers_crossterm::SHIFT => KeyModifiers::SHIFT,
        KeyModifiers_crossterm::CONTROL => KeyModifiers::CONTROL,
        KeyModifiers_crossterm::ALT => KeyModifiers::ALT,
        KeyModifiers_crossterm::SUPER => KeyModifiers::SUPER,
        KeyModifiers_crossterm::HYPER => KeyModifiers::HYPER,
        KeyModifiers_crossterm::META => KeyModifiers::META,
        KeyModifiers_crossterm::NONE => KeyModifiers::NONE,
        _ => todo!(),
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
