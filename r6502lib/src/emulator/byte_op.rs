use crate::emulator::{Cpu, OpCycles};

pub(crate) type ByteOpFn = fn(&mut Cpu, u8) -> OpCycles;

#[derive(Clone)]
pub struct ByteOp(ByteOpFn);

impl ByteOp {
    pub(crate) const fn new(f: ByteOpFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, cpu: &mut Cpu, value: u8) -> OpCycles {
        self.0(cpu, value)
    }
}
