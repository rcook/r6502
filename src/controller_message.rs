use crate::Instruction;

pub(crate) enum ControllerMessage {
    Status(String),
    WriteStdout(char),
    Current(Instruction),
    Disassembly(Instruction),
    Registers(String),
    Cycles(String),
    OnHalted,
    Step,
    Run,
    Break,
}
