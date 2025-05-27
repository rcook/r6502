use crate::{Instruction, RegisterFile, Status};

pub(crate) enum ControllerMessage {
    Status(Status),
    WriteStdout(char),
    Current(Instruction),
    Disassembly(Instruction),
    Registers(RegisterFile),
    Cycles(String),
    Step,
    Run,
    Break,
}
