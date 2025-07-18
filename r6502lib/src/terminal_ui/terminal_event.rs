use crate::emulator::Channel;

#[derive(Debug)]
pub enum TerminalEvent {
    Shutdown,
}

pub type TerminalChannel = Channel<TerminalEvent>;
