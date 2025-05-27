use crate::{Instruction, RegisterFile, Status};

pub(crate) enum StatusMessage {
    BeforeExecute(RegisterFile, u32, Instruction),
    AfterExecute(RegisterFile, u32, Instruction),
    Status(Status),
    WriteStdout(char),
}
