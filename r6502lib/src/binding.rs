use crate::{ByteOp, NoOperandOp, WordOp};

pub(crate) enum Binding {
    NoOperand(NoOperandOp),
    Byte(ByteOp, u8),
    Word(WordOp, u16),
}
