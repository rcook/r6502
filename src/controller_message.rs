pub(crate) enum ControllerMessage {
    WriteStdout(char),
    Current(String),
    History(String),
    Registers(String),
    Step,
    Run,
    Break,
}
