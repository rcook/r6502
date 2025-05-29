use super::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#INXY
// http://www.6502.org/users/obelisk/6502/reference.html#INY
pub(crate) fn iny(s: &mut VmState) -> Cycles {
    let value = s.reg.y.wrapping_add(1);
    s.reg.y = value;
    set_flags_on_value(s, value);
    2
}
