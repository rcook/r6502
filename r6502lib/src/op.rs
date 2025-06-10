use crate::{ByteOp, Cpu, NoOperandOp, OpCycles, WordOp};

#[derive(Clone)]
pub(crate) enum Op {
    NoOperand(NoOperandOp),
    Byte(ByteOp),
    Word(WordOp),
}

impl Op {
    pub(crate) fn execute_no_operand(&self, cpu: &mut Cpu) -> OpCycles {
        match self {
            Self::NoOperand(op) => op.execute(cpu),
            _ => unimplemented!("Cannot execute with no operand"),
        }
    }
}
