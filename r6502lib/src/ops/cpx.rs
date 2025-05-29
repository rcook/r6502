use crate::ops::helper::set_flags_on_compare;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#CPX
// http://www.6502.org/users/obelisk/6502/reference.html#CPX
pub(crate) fn cpx(s: &mut VmState, operand: u8) -> Cycles {
    let result = s.reg.x.wrapping_sub(operand);
    set_flags_on_compare(s, result);
    2
}
