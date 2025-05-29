use super::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#TYA
// http://www.6502.org/users/obelisk/6502/reference.html#TYA
pub(crate) fn tya(s: &mut VmState) -> Cycles {
    let value = s.reg.y;
    s.reg.x = value;
    set_flags_on_value(s, value);
    2
}
