use crate::emulator::Cpu;
use r6502cpu::OpCycles;

pub type NoOperandFn = fn(&mut Cpu) -> OpCycles;

#[derive(Clone)]
pub struct NoOperandOp(NoOperandFn);

impl NoOperandOp {
    pub const fn new(f: NoOperandFn) -> Self {
        Self(f)
    }

    pub fn execute(&self, cpu: &mut Cpu) -> OpCycles {
        self.0(cpu)
    }
}
