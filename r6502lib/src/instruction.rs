use crate::{make_word, Binding, Cpu, Cycles, Op, OpInfo, Opcode, VmState};

#[allow(unused)]
pub(crate) struct Instruction {
    pub(crate) pc: u16,
    pub(crate) opcode: Opcode,
    pub(crate) binding: Binding,
}

impl Instruction {
    pub(crate) fn fetch(cpu: &Cpu, s: &mut VmState) -> Self {
        let value = s.memory[s.reg.pc];
        match Opcode::from_u8(value) {
            Some(opcode) => match cpu.get_op_info(&opcode) {
                Some(OpInfo {
                    opcode: _,
                    addressing_mode: _,
                    op,
                }) => match op {
                    Op::NoOperand { f } => Self {
                        pc: s.reg.pc,
                        opcode,
                        binding: Binding::NoOperand(*f),
                    },
                    Op::Byte(f) => Self {
                        pc: s.reg.pc,
                        opcode,
                        binding: Binding::Byte(f.clone(), s.memory[s.reg.pc + 1]),
                    },
                    Op::Word(f) => Self {
                        pc: s.reg.pc,
                        opcode,
                        binding: Binding::Word(
                            f.clone(),
                            make_word(s.memory[s.reg.pc + 2], s.memory[s.reg.pc + 1]),
                        ),
                    },
                },
                None => unimplemented!("Unsupported opcode ${value:02X}"),
            },
            None => unimplemented!("Invalid opcode ${value:02X}"),
        }
    }

    pub(crate) fn execute(&self, s: &mut VmState) -> Cycles {
        match &self.binding {
            Binding::NoOperand(f) => {
                s.reg.pc += 1;
                f(s)
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
