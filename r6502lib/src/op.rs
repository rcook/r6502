use crate::{ByteOp, CpuState, NoOperandOp, OpCycles, WordOp};

#[derive(Clone)]
pub(crate) enum Op {
    NoOperand(NoOperandOp),
    Byte(ByteOp),
    Word(WordOp),
}

impl Op {
    pub(crate) fn execute_no_operand(&self, state: &mut CpuState) -> OpCycles {
        match self {
            Self::NoOperand(op) => op.execute(state),
            _ => unimplemented!("Cannot execute with no operand"),
        }
    }
}
