use crate::emulator::util::make_word;
use crate::emulator::{Binding, Cpu, Op, OpCycles, Opcode, MOS_6502};

pub struct Instruction {
    pub pc: u16,
    pub opcode: Opcode,
    pub binding: Binding,
}

impl Instruction {
    pub fn fetch(cpu: &Cpu) -> Self {
        let value = cpu.bus.load(cpu.reg.pc);
        match Opcode::from_u8(value) {
            Some(opcode) => match MOS_6502.get_op_info(&opcode) {
                Some(op_info) => match op_info.op() {
                    Op::NoOperand(inner) => Self {
                        pc: cpu.reg.pc,
                        opcode,
                        binding: Binding::NoOperand(inner.clone()),
                    },
                    Op::Byte(inner) => Self {
                        pc: cpu.reg.pc,
                        opcode,
                        binding: Binding::Byte(
                            inner.clone(),
                            cpu.bus.load(cpu.reg.pc.wrapping_add(1)),
                        ),
                    },
                    Op::Word(inner) => Self {
                        pc: cpu.reg.pc,
                        opcode,
                        binding: Binding::Word(
                            inner.clone(),
                            make_word(
                                cpu.bus.load(cpu.reg.pc.wrapping_add(2)),
                                cpu.bus.load(cpu.reg.pc.wrapping_add(1)),
                            ),
                        ),
                    },
                },
                None => unimplemented!("Unsupported opcode ${value:02X}"),
            },
            None => unimplemented!("Invalid opcode ${value:02X}"),
        }
    }

    pub fn execute(&self, cpu: &mut Cpu) -> OpCycles {
        match &self.binding {
            Binding::NoOperand(f) => {
                cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
                f.execute(cpu)
            }
            Binding::Byte(f, value) => {
                cpu.reg.pc = cpu.reg.pc.wrapping_add(2);
                f.execute(cpu, *value)
            }
            Binding::Word(f, value) => {
                cpu.reg.pc = cpu.reg.pc.wrapping_add(3);
                f.execute(cpu, *value)
            }
        }
    }
}
