use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#NOP
// http://www.6502.org/users/obelisk/6502/reference.html#NOP
pub(crate) fn nop(_s: &mut VmState) -> Cycles {
    2
}
