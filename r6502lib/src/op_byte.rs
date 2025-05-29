use crate::{Cycles, VmState};

pub(crate) type OpByteFn = fn(s: &mut VmState, operand: u8) -> Cycles;

pub(crate) type Wrapper = fn(s: &mut VmState, operand: u8) -> (u8, Cycles);

#[derive(Clone)]
pub(crate) enum OpByte {
    Simple { f: OpByteFn },
    Wrapped { wrapper: Wrapper, f: OpByteFn },
}

impl OpByte {
    pub(crate) fn execute(&self, s: &mut VmState, operand: &u8) -> Cycles {
        match self {
            Self::Simple { f } => f(s, *operand),
            Self::Wrapped { wrapper, f } => {
                let (value, extra_cycles) = wrapper(s, *operand);
                let cycles = f(s, value);
                cycles + extra_cycles
            }
        }
    }
}

pub(crate) fn zero_page(s: &mut VmState, operand: u8) -> (u8, Cycles) {
    (s.memory[operand as u16], 1)
}
