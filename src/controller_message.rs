pub(crate) enum ControllerMessage {
    WriteStdout(char),
    Current(String),
    History(String),
    Registers(String),
    Cycles(String),
    OnHalted,
    Step,
    Run,
    Break,
}
