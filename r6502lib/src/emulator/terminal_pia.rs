use crate::emulator::PiaBehaviour;
use cursive::backends::crossterm::crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct TerminalPia;

impl PiaBehaviour for TerminalPia {
    fn enable_raw_mode() {
        enable_raw_mode().expect("Must succeed");
    }

    fn disable_raw_mode() {
        disable_raw_mode().expect("Must succeed");
    }
}
