use crate::{ByteFn, NoOperandFn, Op, WordFn};

pub(crate) enum Instruction {
    NoOperand(Op, NoOperandFn),
    Byte(Op, ByteFn, u8),
    Word(Op, WordFn, u16),
}
