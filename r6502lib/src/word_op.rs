use crate::{Cycles, VmState};

pub(crate) type WordOpFn = fn(&mut VmState, u16) -> Cycles;

#[derive(Clone)]
pub struct WordOp(WordOpFn);

impl WordOp {
    pub(crate) const fn new(f: WordOpFn) -> Self {
        Self(f)
    }

    pub(crate) fn execute(&self, s: &mut VmState, value: &u16) -> Cycles {
        self.0(s, *value)
    }
}
