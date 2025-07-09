use crate::emulator::Channel;
use cursive::backends::crossterm::crossterm::event::Event;

#[derive(Debug)]
pub enum IoEvent {
    PaUpdated(u8),
    PacrUpdated(u8),
    PbUpdated(u8),
    PbcrUpdated(u8),
    Input(Event),
    Shutdown,
}

pub type IoChannel = Channel<IoEvent>;
