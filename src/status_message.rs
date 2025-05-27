use crate::{Instruction, RegisterFile, Status};

pub(crate) enum StatusMessage {
    BeforeExecute(RegisterFile, u32, Instruction, u16),
    AfterExecute(RegisterFile, u32, Instruction),
    Status(Status),
    WriteStdout(char),
}
