use crate::emulator::PiaBehaviour;
use anyhow::Result;
use cursive::backends::crossterm::crossterm::event::{poll, read, Event};
use cursive::backends::crossterm::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::time::Duration;

pub struct TerminalPia;

impl TerminalPia {
    fn is_event_available() -> Result<bool> {
        Ok(poll(Duration::from_millis(100))?)
    }
}

impl PiaBehaviour for TerminalPia {
    fn enable_raw_mode() {
        enable_raw_mode().expect("Must succeed");
    }

    fn disable_raw_mode() {
        disable_raw_mode().expect("Must succeed");
    }

    fn try_read_event() -> Result<Option<Event>> {
        if Self::is_event_available()? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }
}
