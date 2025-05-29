use crate::{Cycles, OpByteFn, VmState};

pub(crate) type OpWordFn = fn(s: &mut VmState, operand: u16) -> Cycles;

pub(crate) type Wrapper = fn(s: &mut VmState, operand: u16) -> (u8, Cycles);

#[derive(Clone)]
pub(crate) enum OpWord {
    Simple { f: OpWordFn },
    Wrapped { wrapper: Wrapper, f: OpByteFn },
}

impl OpWord {
    pub(crate) fn execute(&self, s: &mut VmState, operand: &u16) -> Cycles {
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

pub(crate) fn absolute(s: &mut VmState, operand: u16) -> (u8, Cycles) {
    (s.memory[operand], 2)
}
