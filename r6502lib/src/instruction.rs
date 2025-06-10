use crate::util::make_word;
use crate::{Binding, Cpu, Op, OpCycles, Opcode, MOS_6502};

pub(crate) struct Instruction {
    pub(crate) pc: u16,
    pub(crate) opcode: Opcode,
    pub(crate) binding: Binding,
}

impl Instruction {
    pub(crate) fn fetch(s: &Cpu) -> Self {
        let value = s.memory.load(s.reg.pc);
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
                        binding: Binding::Byte(
                            inner.clone(),
                            s.memory.load(s.reg.pc.wrapping_add(1)),
                        ),
                    },
                    Op::Word(inner) => Self {
                        pc: s.reg.pc,
                        opcode,
                        binding: Binding::Word(
                            inner.clone(),
                            make_word(
                                s.memory.load(s.reg.pc.wrapping_add(2)),
                                s.memory.load(s.reg.pc.wrapping_add(1)),
                            ),
                        ),
                    },
                },
                None => unimplemented!("Unsupported opcode ${value:02X}"),
            },
            None => unimplemented!("Invalid opcode ${value:02X}"),
        }
    }

    pub(crate) fn execute(&self, state: &mut Cpu) -> OpCycles {
        match &self.binding {
            Binding::NoOperand(f) => {
                state.reg.pc = state.reg.pc.wrapping_add(1);
                f.execute(state)
            }
            Binding::Byte(f, value) => {
                state.reg.pc = state.reg.pc.wrapping_add(2);
                f.execute(state, value)
            }
            Binding::Word(f, value) => {
                state.reg.pc = state.reg.pc.wrapping_add(3);
                f.execute(state, value)
            }
        }
    }
}
