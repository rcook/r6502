use crate::{Cpu, OpCycles};

pub(crate) type WordOpFn = fn(&mut Cpu, u16) -> OpCycles;

#[derive(Clone)]
pub struct WordOp(WordOpFn);

impl WordOp {
    pub(crate) const fn new(f: WordOpFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, state: &mut Cpu, value: &u16) -> OpCycles {
        self.0(state, *value)
    }
}
