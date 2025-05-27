use crate::Instruction;

pub(crate) enum UIMessage {
    Status(String),
    WriteStdout(char),
    Current(Instruction),
    Disassembly(Instruction),
    Registers(String),
    Cycles(String),
}
