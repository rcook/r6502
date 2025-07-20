pub enum TerminalCommand {
    Backspace,
    ClearLine,
    ClearScreen,
    NewLine,
    UpdateCursor,
    Write(String),
}
