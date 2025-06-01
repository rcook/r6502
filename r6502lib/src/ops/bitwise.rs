use crate::ops::helper::set_flags_on_value;
use crate::{p_set, OpCycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#AND
// http://www.6502.org/users/obelisk/6502/reference.html#AND
pub(crate) fn and(s: &mut VmState, operand: u8) -> OpCycles {
    s.reg.a &= operand;
    set_flags_on_value(s, s.reg.a);
    2
}

// http://www.6502.org/tutorials/6502opcodes.html#BIT
// http://www.6502.org/users/obelisk/6502/reference.html#BIT
pub(crate) fn bit(s: &mut VmState, operand: u8) -> OpCycles {
    let value = s.reg.a & operand;
    set_flags_on_value(s, s.reg.a);
    p_set!(s.reg, V, (value & 0b01000000) != 0);
    2
}

// http://www.6502.org/tutorials/6502opcodes.html#EOR
// http://www.6502.org/users/obelisk/6502/reference.html#EOR
pub(crate) fn eor(s: &mut VmState, operand: u8) -> OpCycles {
    s.reg.a ^= operand;
    set_flags_on_value(s, s.reg.a);
    2
}

// http://www.6502.org/tutorials/6502opcodes.html#ORA
// http://www.6502.org/users/obelisk/6502/reference.html#ORA
pub(crate) fn ora(s: &mut VmState, operand: u8) -> OpCycles {
    s.reg.a |= operand;
    set_flags_on_value(s, s.reg.a);
    2
}
