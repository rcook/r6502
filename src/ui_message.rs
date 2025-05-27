use crate::{Instruction, RegisterFile, Status};

pub(crate) enum UIMessage {
    Status(Status),
    WriteStdout(char),
    Current(Instruction),
    Disassembly(Instruction),
    Registers(RegisterFile),
    Cycles(u32),
}
