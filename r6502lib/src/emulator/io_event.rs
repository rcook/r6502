use r6502core::Channel;
use r6502core::events::Event;

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
