use super::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#INX
// http://www.6502.org/users/obelisk/6502/reference.html#INX
pub(crate) fn inx(s: &mut VmState) -> Cycles {
    let value = s.reg.x.wrapping_add(1);
    s.reg.x = value;
    set_flags_on_value(s, value);
    2
}
