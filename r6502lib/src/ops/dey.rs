use super::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#DEY
// http://www.6502.org/users/obelisk/6502/reference.html#DEY
pub(crate) fn dey(s: &mut VmState) -> Cycles {
    let value = s.reg.y.wrapping_sub(1);
    s.reg.y = value;
    set_flags_on_value(s, value);
    2
}
