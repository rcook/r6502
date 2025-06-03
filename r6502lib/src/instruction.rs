use crate::util::make_word;
use crate::{Binding, Op, OpCycles, Opcode, VmState, MOS_6502};

#[allow(unused)]
pub(crate) struct Instruction {
    pub(crate) pc: u16,
    pub(crate) opcode: Opcode,
    pub(crate) binding: Binding,
}

impl Instruction {
    pub(crate) fn fetch(s: &VmState) -> Self {
        let value = s.memory[s.reg.pc];
        match Opcode::from_u8(value) {
            Some(opcode) => match MOS_6502.get_op_info(&opcode) {
                Some(op_info) => match op_info.op() {
                    Op::NoOperand(inner) => Self {
                        pc: s.reg.pc,
                        opcode,
                        binding: Binding::NoOperand(inner.clone()),
                    },
                    Op::Byte(inner) => Self {
                        pc: s.reg.pc,
                        opcode,
                        binding: Binding::Byte(inner.clone(), s.memory[s.reg.pc + 1]),
                    },
                    Op::Word(inner) => Self {
                        pc: s.reg.pc,
                        opcode,
                        binding: Binding::Word(
                            inner.clone(),
                            make_word(s.memory[s.reg.pc + 2], s.memory[s.reg.pc + 1]),
                        ),
                    },
                },
                None => unimplemented!("Unsupported opcode ${value:02X}"),
            },
            None => unimplemented!("Invalid opcode ${value:02X}"),
        }
    }

    pub(crate) fn execute(&self, s: &mut VmState) -> OpCycles {
        match &self.binding {
            Binding::NoOperand(f) => {
                println!("BEFORE {:04X}", s.reg.pc);
                s.reg.pc += 1;
                println!("AFTER {:04X}", s.reg.pc);
                f.execute(s)
            }
            Binding::Byte(f, value) => {
                s.reg.pc += 2;
                f.execute(s, value)
            }
            Binding::Word(f, value) => {
                s.reg.pc += 3;
                f.execute(s, value)
            }
        }
    }
}
