pub(crate) enum UIMessage {
    WriteStdout(char),
    Println(String),
    ShowRegisters(String),
}
