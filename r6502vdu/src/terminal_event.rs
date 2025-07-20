use r6502lib::keyboard::KeyEvent;

#[derive(Debug)]
pub enum TerminalEvent {
    Closed,
    Key(KeyEvent),
    TextInput(String),
}
