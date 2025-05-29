use crate::ops::helper::branch;
use crate::{Cycles, VmState, P};

// http://www.6502.org/tutorials/6502opcodes.html#BCS
// http://www.6502.org/users/obelisk/6502/reference.html#BCS
pub(crate) fn bcs(s: &mut VmState, operand: u8) -> Cycles {
    branch(s, operand, P::C, true)
}
