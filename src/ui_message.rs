pub(crate) enum UIMessage {
    WriteStdout(char),
    Current(String),
    Disassembly(String),
    Registers(String),
    Cycles(String),
}
