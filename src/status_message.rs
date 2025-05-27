use crate::{Cycles, Instruction, RegisterFile, Status};

pub(crate) enum StatusMessage {
    BeforeExecute {
        reg: RegisterFile,
        cycles: Cycles,
        instruction: Instruction,
    },
    AfterExecute {
        reg: RegisterFile,
        cycles: Cycles,
        instruction: Instruction,
    },
    Status(Status),
    WriteStdout(char),
}
