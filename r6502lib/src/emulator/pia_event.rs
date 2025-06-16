use crate::emulator::Channel;
use cursive::backends::crossterm::crossterm::event::Event;

#[derive(Debug)]
pub enum PiaEvent {
    PaUpdated,
    PacrUpdated,
    PbUpdated,
    PbcrUpdated,
    Input(Event),
    Shutdown,
}

pub type PiaChannel = Channel<PiaEvent>;
