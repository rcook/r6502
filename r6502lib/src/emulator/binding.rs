use crate::emulator::{ByteOp, NoOperandOp, WordOp};

pub enum Binding {
    NoOperand(NoOperandOp),
    Byte(ByteOp, u8),
    Word(WordOp, u16),
}
