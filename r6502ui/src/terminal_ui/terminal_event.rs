use r6502core::Channel;

#[derive(Debug)]
pub enum TerminalEvent {
    Shutdown,
}

pub type TerminalChannel = Channel<TerminalEvent>;
