use r6502lib::Channel;

#[derive(Debug)]
pub enum TerminalEvent {
    Shutdown,
}

pub type TerminalChannel = Channel<TerminalEvent>;
