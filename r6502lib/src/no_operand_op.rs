use crate::{CpuState, OpCycles};

pub(crate) type NoOperandFn = fn(&mut CpuState) -> OpCycles;

#[derive(Clone)]
pub struct NoOperandOp(NoOperandFn);

impl NoOperandOp {
    pub(crate) const fn new(f: NoOperandFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, state: &mut CpuState) -> OpCycles {
        self.0(state)
    }
}
