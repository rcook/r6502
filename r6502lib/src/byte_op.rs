use crate::{CpuState, OpCycles};

pub(crate) type ByteOpFn = fn(&mut CpuState, u8) -> OpCycles;

#[derive(Clone)]
pub struct ByteOp(ByteOpFn);

impl ByteOp {
    pub(crate) const fn new(f: ByteOpFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, state: &mut CpuState, value: &u8) -> OpCycles {
        self.0(state, *value)
    }
}
