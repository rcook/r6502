use super::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#TXA
// http://www.6502.org/users/obelisk/6502/reference.html#TXA
pub(crate) fn txa(s: &mut VmState) -> Cycles {
    let value = s.reg.x;
    s.reg.a = value;
    set_flags_on_value(s, value);
    2
}
