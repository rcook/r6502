use r6502lib::Channel;
use r6502lib::keyboard::KeyEvent;

#[derive(Debug)]
pub enum IoEvent {
    PaUpdated(u8),
    PacrUpdated(u8),
    PbUpdated(u8),
    PbcrUpdated(u8),
    Input(KeyEvent),
    Shutdown,
}

pub type IoChannel = Channel<IoEvent>;
