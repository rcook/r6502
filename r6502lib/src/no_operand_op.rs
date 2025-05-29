use crate::{Cycles, VmState};

pub(crate) type NoOperandFn = fn(&mut VmState) -> Cycles;

#[derive(Clone)]
pub(crate) struct NoOperandOp {
    pub(crate) f: NoOperandFn,
}

impl NoOperandOp {
    pub(crate) fn execute(&self, s: &mut VmState) -> Cycles {
        (self.f)(s)
    }
}
