use anyhow::Result;
use cursive::backends::crossterm::crossterm::event::Event;

pub trait PiaBehaviour {
    fn enable_raw_mode() {}
    fn disable_raw_mode() {}
    fn try_read_event() -> Result<Option<Event>>;
}
