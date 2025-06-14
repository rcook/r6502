use crate::emulator::{ByteOp, Cpu, NoOperandOp, OpCycles, WordOp};

#[derive(Clone)]
pub enum Op {
    NoOperand(NoOperandOp),
    Byte(ByteOp),
    Word(WordOp),
}

impl Op {
    pub fn execute_no_operand(&self, cpu: &mut Cpu) -> OpCycles {
        match self {
            Self::NoOperand(op) => op.execute(cpu),
            _ => unimplemented!("Cannot execute with no operand"),
        }
    }

    pub fn execute_word(&self, cpu: &mut Cpu, value: u16) -> OpCycles {
        match self {
            Self::Word(op) => op.execute(cpu, value),
            _ => unimplemented!("Cannot execute with no operand"),
        }
    }
}
