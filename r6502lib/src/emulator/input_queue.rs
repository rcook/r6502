use anyhow::Result;
use cursive::backends::crossterm::crossterm::event::Event;
use std::sync::{Arc, Mutex};

pub type InputQueueRef = Arc<Mutex<dyn InputQueue>>;

pub trait InputQueue: Send {
    fn enable_raw_mode(&self) -> Result<()> {
        Ok(())
    }

    fn disable_raw_mode(&self) -> Result<()> {
        Ok(())
    }

    fn try_read_event(&mut self) -> Result<Option<Event>>;
}
