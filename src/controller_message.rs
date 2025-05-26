pub(crate) enum ControllerMessage {
    WriteStdout(char),
    Println(String),
    ShowRegisters(String),
    Step,
    Run,
    Break,
}
