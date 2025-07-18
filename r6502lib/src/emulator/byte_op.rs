use crate::emulator::Cpu;
use r6502cpu::OpCycles;

pub type ByteOpFn = fn(&mut Cpu, u8) -> OpCycles;

#[derive(Clone)]
pub struct ByteOp(ByteOpFn);

impl ByteOp {
    pub const fn new(f: ByteOpFn) -> Self {
        Self(f)
    }

    pub fn execute(&self, cpu: &mut Cpu, value: u8) -> OpCycles {
        self.0(cpu, value)
    }
}
