use crate::{Instruction, RegisterFile};

pub(crate) enum UIMessage {
    Status(String),
    WriteStdout(char),
    Current(Instruction),
    Disassembly(Instruction),
    Registers(RegisterFile),
    Cycles(String),
}
