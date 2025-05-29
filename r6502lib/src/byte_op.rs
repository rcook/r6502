use crate::{Cycles, VmState};

pub(crate) type ByteOpFn = fn(&mut VmState, u8) -> Cycles;

#[derive(Clone)]
pub(crate) struct ByteOp(ByteOpFn);

impl ByteOp {
    pub(crate) const fn new(f: ByteOpFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, s: &mut VmState, value: &u8) -> Cycles {
        self.0(s, *value)
    }
}
