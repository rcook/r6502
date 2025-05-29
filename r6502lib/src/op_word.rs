use crate::{Cycles, OpByteFn, VmState};
use std::rc::Rc;

pub(crate) type OpWordFn = fn(s: &mut VmState, operand: u16) -> Cycles;

pub(crate) type OpWordClosure = Rc<Box<dyn Fn(&mut VmState, u16) -> Cycles>>;

#[derive(Clone)]
pub(crate) enum OpWord {
    Simple { f: OpWordFn },
    Wrapped { f: OpWordClosure },
}

impl OpWord {
    pub(crate) fn execute(&self, s: &mut VmState, operand: &u16) -> Cycles {
        match self {
            Self::Simple { f } => f(s, *operand),
            Self::Wrapped { f } => f(s, *operand),
        }
    }
}

pub(crate) fn absolute(f: OpByteFn) -> OpWordClosure {
    Rc::new(Box::new(move |s, operand| {
        let value = s.memory[operand];
        f(s, value) + 2
    }))
}
