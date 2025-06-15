use crate::emulator::InputQueue;
use anyhow::Result;
use cursive::backends::crossterm::crossterm::event::{poll, read, Event};
use cursive::backends::crossterm::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::time::Duration;

pub struct TerminalInputQueue;

impl TerminalInputQueue {
    pub fn new() -> Self {
        Self
    }

    fn is_event_available() -> Result<bool> {
        Ok(poll(Duration::from_millis(100))?)
    }
}

impl InputQueue for TerminalInputQueue {
    fn enable_raw_mode(&self) -> Result<()> {
        Ok(enable_raw_mode()?)
    }

    fn disable_raw_mode(&self) -> Result<()> {
        Ok(disable_raw_mode()?)
    }

    fn try_read_event(&mut self) -> Result<Option<Event>> {
        if Self::is_event_available()? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }
}
