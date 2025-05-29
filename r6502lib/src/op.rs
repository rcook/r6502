use crate::{ByteOp, NoOperandOp, WordOp};

#[derive(Clone)]
pub(crate) enum Op {
    NoOperand(NoOperandOp),
    Byte(ByteOp),
    Word(WordOp),
}
