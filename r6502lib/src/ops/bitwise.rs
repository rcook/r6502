use crate::ops::helper::set_flags_on_value;
use crate::{p_set, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#AND
// http://www.6502.org/users/obelisk/6502/reference.html#AND
pub(crate) fn and(s: &mut VmState, operand: u8) {
    s.reg.a &= operand;
    set_flags_on_value(s, s.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#BIT
// http://www.6502.org/users/obelisk/6502/reference.html#BIT
pub(crate) fn bit(s: &mut VmState, operand: u8) {
    let value = s.reg.a & operand;
    p_set!(s.reg, N, (operand & 0b10000000) != 0);
    p_set!(s.reg, V, (operand & 0b01000000) != 0);
    p_set!(s.reg, Z, value == 0);
}

// http://www.6502.org/tutorials/6502opcodes.html#EOR
// http://www.6502.org/users/obelisk/6502/reference.html#EOR
pub(crate) fn eor(s: &mut VmState, operand: u8) {
    s.reg.a ^= operand;
    set_flags_on_value(s, s.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#ORA
// http://www.6502.org/users/obelisk/6502/reference.html#ORA
pub(crate) fn ora(s: &mut VmState, operand: u8) {
    s.reg.a |= operand;
    set_flags_on_value(s, s.reg.a);
}
