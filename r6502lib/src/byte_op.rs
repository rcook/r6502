use crate::{Cycles, VmState};

pub(crate) type ByteOpFn = fn(&mut VmState, u8) -> Cycles;

pub(crate) type Wrapper = fn(&mut VmState, u8) -> (u8, Cycles);

#[derive(Clone)]
pub(crate) enum ByteOp {
    Simple { f: ByteOpFn },
    Wrapped { wrapper: Wrapper, f: ByteOpFn },
}

impl ByteOp {
    pub(crate) fn execute(&self, s: &mut VmState, value: &u8) -> Cycles {
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

pub(crate) fn zero_page(s: &mut VmState, value: u8) -> (u8, Cycles) {
    (s.memory[value as u16], 1)
}
