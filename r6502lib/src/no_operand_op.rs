use crate::{OpCycles, VmState};

pub(crate) type NoOperandFn = fn(&mut VmState) -> OpCycles;

#[derive(Clone)]
pub struct NoOperandOp(NoOperandFn);

impl NoOperandOp {
    pub(crate) const fn new(f: NoOperandFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, s: &mut VmState) -> OpCycles {
        self.0(s)
    }
}
