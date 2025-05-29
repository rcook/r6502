use crate::{make_word, Cpu, Cycles, Op, OpByte, OpNoOperandFn, OpWord, Opcode, VmState};

pub(crate) enum Instruction {
    NoOperand { f: OpNoOperandFn },
    Byte { f: OpByte, operand: u8 },
    Word { f: OpWord, operand: u16 },
}

impl Instruction {
    pub(crate) fn fetch(cpu: &Cpu, s: &mut VmState) -> Self {
        let value = s.memory[s.reg.pc];
        match Opcode::from_u8(value) {
            Some(opcode) => match cpu.get_op(&opcode) {
                Some(op) => match op {
                    Op::NoOperand { f } => Self::NoOperand { f: *f },
                    Op::Byte(f) => Self::Byte {
                        f: f.clone(),
                        operand: s.memory[s.reg.pc + 1],
                    },
                    Op::Word(f) => Self::Word {
                        f: f.clone(),
                        operand: make_word(s.memory[s.reg.pc + 2], s.memory[s.reg.pc + 1]),
                    },
                },
                None => unimplemented!("Unsupported opcode ${value:02X}"),
            },
            None => unimplemented!("Invalid opcode ${value:02X}"),
        }
    }

    pub(crate) fn execute(&self, s: &mut VmState) -> Cycles {
        match self {
            Self::NoOperand { f } => {
                s.reg.pc += 1;
                f(s)
            }
            Self::Byte { f, operand } => {
                s.reg.pc += 2;
                f.execute(s, operand)
            }
            Self::Word { f, operand } => {
                s.reg.pc += 3;
                f.execute(s, operand)
            }
        }
    }
}
