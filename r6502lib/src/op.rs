use crate::{Cycles, OpByte, OpWord, VmState};

pub(crate) type OpNoOperandFn = fn(s: &mut VmState) -> Cycles;

pub(crate) enum Op {
    NoOperand { f: OpNoOperandFn },
    Byte(OpByte),
    Word(OpWord),
}
