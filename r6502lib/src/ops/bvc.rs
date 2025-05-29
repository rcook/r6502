use crate::ops::helper::branch;
use crate::{Cycles, VmState, P};

// http://www.6502.org/tutorials/6502opcodes.html#BVC
// http://www.6502.org/users/obelisk/6502/reference.html#BVC
pub(crate) fn bvc(s: &mut VmState, operand: u8) -> Cycles {
    branch(s, operand, P::V, false)
}
