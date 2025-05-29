use crate::{Cycles, VmState};
use std::rc::Rc;

pub(crate) type OpByteFn = fn(s: &mut VmState, operand: u8) -> Cycles;

pub(crate) type OpByteClosure = Rc<Box<dyn Fn(&mut VmState, u8) -> Cycles>>;

#[derive(Clone)]
pub(crate) enum OpByte {
    Simple { f: OpByteFn },
    Wrapped { f: OpByteClosure },
}

impl OpByte {
    pub(crate) fn execute(&self, s: &mut VmState, operand: &u8) -> Cycles {
        match self {
            Self::Simple { f } => f(s, *operand),
            Self::Wrapped { f } => f(s, *operand),
        }
    }
}

pub(crate) fn zero_page(f: OpByteFn) -> OpByteClosure {
    Rc::new(Box::new(move |s, operand| {
        let value = s.memory[operand as u16];
        f(s, value) + 1
    }))
}
