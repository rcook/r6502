use crate::{ByteOpFn, Cycles, VmState};

pub(crate) type WordOpFn = fn(&mut VmState, u16) -> Cycles;

pub(crate) type Wrapper = fn(&mut VmState, u16) -> (u8, Cycles);

#[derive(Clone)]
pub(crate) enum WordOp {
    Simple { f: WordOpFn },
    Wrapped { wrapper: Wrapper, f: ByteOpFn },
}

impl WordOp {
    pub(crate) fn execute(&self, s: &mut VmState, value: &u16) -> Cycles {
        match self {
            Self::Simple { f } => f(s, *value),
            Self::Wrapped { wrapper, f } => {
                let (result, extra_cycles) = wrapper(s, *value);
                let cycles = f(s, result);
                cycles + extra_cycles
            }
        }
    }
}

pub(crate) fn absolute(s: &mut VmState, value: u16) -> (u8, Cycles) {
    (s.memory[value], 2)
}
