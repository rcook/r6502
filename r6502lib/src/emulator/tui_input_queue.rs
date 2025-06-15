use crate::emulator::InputQueue;
use anyhow::Result;
use cursive::backends::crossterm::crossterm::event::Event;
use std::collections::VecDeque;

pub struct TuiInputQueue {
    events: VecDeque<Event>,
}

impl TuiInputQueue {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push_back(event);
    }
}

impl InputQueue for TuiInputQueue {
    fn try_read_event(&mut self) -> Result<Option<Event>> {
        Ok(self.events.pop_front())
    }
}
