use crate::emulator::PiaBehaviour;
use anyhow::Result;
use cursive::backends::crossterm::crossterm::event::Event;

pub struct TuiPia;

impl PiaBehaviour for TuiPia {
    fn try_read_event() -> Result<Option<Event>> {
        Ok(None)
    }
}
