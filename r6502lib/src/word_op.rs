use crate::{OpCycles, VmState};

pub(crate) type WordOpFn = fn(&mut VmState, u16) -> OpCycles;

#[derive(Clone)]
pub struct WordOp(WordOpFn);

impl WordOp {
    pub(crate) const fn new(f: WordOpFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, s: &mut VmState, value: &u16) -> OpCycles {
        self.0(s, *value)
    }
}
