use crate::{ByteOp, NoOperandOp, OpCycles, VmState, WordOp};

#[derive(Clone)]
pub(crate) enum Op {
    NoOperand(NoOperandOp),
    Byte(ByteOp),
    Word(WordOp),
}

impl Op {
    pub(crate) fn execute_no_operand(&self, s: &mut VmState) -> OpCycles {
        match self {
            Self::NoOperand(op) => op.execute(s),
            _ => unimplemented!("Cannot execute with no operand"),
        }
    }
}
