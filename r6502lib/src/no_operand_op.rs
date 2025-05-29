use crate::{Cycles, VmState};

pub(crate) type NoOperandFn = fn(&mut VmState) -> Cycles;

#[derive(Clone)]
pub(crate) struct NoOperandOp(NoOperandFn);

impl NoOperandOp {
    pub(crate) const fn new(f: NoOperandFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, s: &mut VmState) -> Cycles {
        self.0(s)
    }
}
