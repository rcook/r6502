use crate::{ByteOp, Cycles, NoOperandOp, VmState, WordOp};

#[derive(Clone)]
pub(crate) enum Op {
    NoOperand(NoOperandOp),
    Byte(ByteOp),
    Word(WordOp),
}

impl Op {
    #[allow(unused)]
    pub(crate) fn execute_no_operand(&self, s: &mut VmState) -> Cycles {
        match self {
            Self::NoOperand(op) => op.execute(s),
            _ => unimplemented!("Cannot execute with no operand"),
        }
    }
}
