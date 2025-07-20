use log::warn;
use r6502lib::keyboard::{KeyCode, KeyEvent, KeyModifiers};
use sdl3::keyboard::{Keycode, Mod};

pub fn normalize_keymod(value: Mod) -> Mod {
    // Always ignore NUMMOD
    value & !Mod::NUMMOD
}

#[allow(unused)]
pub fn translate_key_event(keycode: Keycode, keymod: Mod) -> Option<KeyEvent> {
    Some(KeyEvent {
        code: translate_key_code(keycode)?,
        modifiers: translate_key_modifiers(keymod),
    })
}

pub fn translate_key_code(keycode: Keycode) -> Option<KeyCode> {
    match keycode {
        Keycode::Backspace => Some(KeyCode::Backspace),
        Keycode::Delete => Some(KeyCode::Delete),
        Keycode::Escape => Some(KeyCode::Esc),
        Keycode::Return => Some(KeyCode::Enter),
        _ => None,
    }
}

pub fn translate_key_modifiers(keymod: Mod) -> KeyModifiers {
    fn update(modifiers: &mut KeyModifiers, value: Mod) {
        match value {
            Mod::NOMOD => {}
            Mod::LSHIFTMOD | Mod::RSHIFTMOD => *modifiers |= KeyModifiers::SHIFT,
            Mod::LCTRLMOD | Mod::RCTRLMOD => *modifiers |= KeyModifiers::CONTROL,
            Mod::LALTMOD | Mod::RALTMOD => *modifiers |= KeyModifiers::ALT,
            _ => warn!("unimplemented key modifier: {value:?}"),
        }
    }

    let mut modifiers = KeyModifiers::NONE;
    for value in keymod {
        update(&mut modifiers, value);
    }

    modifiers
}
