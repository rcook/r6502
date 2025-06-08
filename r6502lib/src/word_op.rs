use crate::{CpuState, OpCycles};

pub(crate) type WordOpFn = fn(&mut CpuState, u16) -> OpCycles;

#[derive(Clone)]
pub struct WordOp(WordOpFn);

impl WordOp {
    pub(crate) const fn new(f: WordOpFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, state: &mut CpuState, value: &u16) -> OpCycles {
        self.0(state, *value)
    }
}
