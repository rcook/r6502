use crate::emulator::{ByteOp, NoOperandOp, WordOp};

pub(crate) enum Binding {
    NoOperand(NoOperandOp),
    Byte(ByteOp, u8),
    Word(WordOp, u16),
}
