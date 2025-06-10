use crate::{Cpu, OpCycles};

pub(crate) type NoOperandFn = fn(&mut Cpu) -> OpCycles;

#[derive(Clone)]
pub struct NoOperandOp(NoOperandFn);

impl NoOperandOp {
    pub(crate) const fn new(f: NoOperandFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, state: &mut Cpu) -> OpCycles {
        self.0(state)
    }
}
