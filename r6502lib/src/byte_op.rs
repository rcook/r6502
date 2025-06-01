use crate::{OpCycles, VmState};

pub(crate) type ByteOpFn = fn(&mut VmState, u8) -> OpCycles;

#[derive(Clone)]
pub struct ByteOp(ByteOpFn);

impl ByteOp {
    pub(crate) const fn new(f: ByteOpFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, s: &mut VmState, value: &u8) -> OpCycles {
        self.0(s, *value)
    }
}
