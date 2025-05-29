use crate::{OpByte, OpNoOperandFn, OpWord};

#[allow(unused)]
pub(crate) enum Binding {
    NoOperand(OpNoOperandFn),
    Byte(OpByte, u8),
    Word(OpWord, u16),
}
