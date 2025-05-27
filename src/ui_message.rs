pub(crate) enum UIMessage {
    WriteStdout(char),
    Current(String),
    History(String),
    Registers(String),
    Cycles(String),
}
