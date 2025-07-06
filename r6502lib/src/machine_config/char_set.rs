use crate::ascii::{BS, CR, DEL, ESC, LF};
use cursive::backends::crossterm::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::info;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub enum CharSet {
    #[default]
    #[serde(rename = "default")]
    Default,

    #[serde(rename = "acorn")]
    Acorn,

    #[serde(rename = "apple1")]
    Apple1,
}

impl CharSet {
    #[must_use]
    pub fn translate_in(&self, key: &KeyEvent) -> Option<u8> {
        let c = match (self, key.modifiers, key.code) {
            (_, KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => c,
            (Self::Acorn, KeyModifiers::NONE, KeyCode::Backspace | KeyCode::Delete) => {
                return Some(DEL)
            }
            (_, KeyModifiers::NONE, KeyCode::Backspace | KeyCode::Delete) => '_',
            (_, KeyModifiers::NONE, KeyCode::Enter) => CR as char,
            (_, KeyModifiers::NONE, KeyCode::Esc) => ESC as char,
            _ => return None,
        };
        Some(self.to_byte(c))
    }

    #[must_use]
    pub fn translate_out(&self, value: u8) -> Option<u8> {
        match self {
            Self::Default => Some(value),
            Self::Acorn => {
                match value {
                    CR => None, // Swallow CR
                    DEL => Some(BS),
                    10 | 32..=126 => Some(value),
                    _ => {
                        info!("nonprinting VDU code: {value:>3} ${value:02X}");
                        Some(value)
                    }
                }
            }
            Self::Apple1 => {
                match value {
                    0x7f => None,            // Filter out initialization
                    0x8d => Some(LF),        // Translate CR with high bit set to LF
                    _ => Some(value & 0x7f), // Clear the high bit
                }
            }
        }
    }

    fn to_byte(self, c: char) -> u8 {
        match self {
            Self::Apple1 => {
                let value = c.to_ascii_uppercase() as u8;
                assert_ne!(0, value);
                value | 0x80
            }
            _ => c as u8,
        }
    }
}
