use log::info;
use r6502config::CharSet;
use r6502core::ascii::{BEL, BS, CR, DEL, ESC, LF};
use r6502core::keyboard::{KeyCode, KeyEvent, KeyModifiers};

#[must_use]
pub fn translate_in(char_set: &CharSet, key: &KeyEvent) -> Option<u8> {
    let c = match (char_set, key.modifiers, key.code) {
        (_, KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => c,
        (CharSet::Acorn, KeyModifiers::NONE, KeyCode::Backspace | KeyCode::Delete) => {
            return Some(DEL);
        }
        (_, KeyModifiers::NONE, KeyCode::Backspace | KeyCode::Delete) => '_',
        (_, KeyModifiers::NONE, KeyCode::Enter) => CR as char,
        (_, KeyModifiers::NONE, KeyCode::Esc) => ESC as char,
        _ => return None,
    };
    Some(to_byte(*char_set, c))
}

#[must_use]
pub fn translate_out(char_set: &CharSet, value: u8) -> Option<u8> {
    match char_set {
        CharSet::Default => Some(value),
        CharSet::Acorn => match value {
            BEL | 32..=126 => Some(value),
            CR => Some(LF),
            DEL => Some(BS),
            ESC | LF => None,
            _ => {
                info!("nonprinting VDU code: {value:>3} (${value:02X})");
                Some(value)
            }
        },
        CharSet::Apple1 => {
            match value {
                0x7f => None,            // Filter out initialization
                0x8d => Some(LF),        // Translate CR with high bit set to LF
                _ => Some(value & 0x7f), // Clear the high bit
            }
        }
    }
}

fn to_byte(char_set: CharSet, c: char) -> u8 {
    match char_set {
        CharSet::Apple1 => {
            let value = c.to_ascii_uppercase() as u8;
            assert_ne!(0, value);
            value | 0x80
        }
        _ => c as u8,
    }
}
