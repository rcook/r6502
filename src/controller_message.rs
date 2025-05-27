use crate::{Instruction, RegisterFile};

pub(crate) enum ControllerMessage {
    Status(String),
    WriteStdout(char),
    Current(Instruction),
    Disassembly(Instruction),
    Registers(RegisterFile),
    Cycles(String),
    OnHalted,
    Step,
    Run,
    Break,
}
