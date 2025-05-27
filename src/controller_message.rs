pub(crate) enum ControllerMessage {
    WriteStdout(char),
    Current(String),
    History(String),
    Registers(String),
    OnHalted,
    Step,
    Run,
    Break,
}
