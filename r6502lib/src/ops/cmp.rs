use crate::ops::helper::set_flags_on_compare;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#CMP
// http://www.6502.org/users/obelisk/6502/reference.html#CMP
pub(crate) fn cmp(s: &mut VmState, operand: u8) -> Cycles {
    let result = s.reg.a.wrapping_sub(operand);
    set_flags_on_compare(s, result);
    2
}
