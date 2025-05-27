pub(crate) enum ControllerMessage {
    WriteStdout(char),
    Current(String),
    Disassembly(String),
    Registers(String),
    Cycles(String),
    OnHalted,
    Step,
    Run,
    Break,
}
