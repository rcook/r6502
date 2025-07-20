use crate::OpCycles;
use crate::emulator::Cpu;

pub type WordOpFn = fn(&mut Cpu, u16) -> OpCycles;

#[derive(Clone)]
pub struct WordOp(WordOpFn);

impl WordOp {
    pub const fn new(f: WordOpFn) -> Self {
        Self(f)
    }

    pub fn execute(&self, cpu: &mut Cpu, value: u16) -> OpCycles {
        self.0(cpu, value)
    }
}
