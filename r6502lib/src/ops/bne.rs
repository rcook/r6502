use crate::ops::helper::branch;
use crate::{Cycles, VmState, P};

// http://www.6502.org/tutorials/6502opcodes.html#BNE
// http://www.6502.org/users/obelisk/6502/reference.html#BNE
pub(crate) fn bne(s: &mut VmState, operand: u8) -> Cycles {
    branch(s, operand, P::Z, false)
}
