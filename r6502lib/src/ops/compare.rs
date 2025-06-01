use crate::ops::helper::set_flags_on_compare;
use crate::VmState;

// http://www.6502.org/tutorials/6502opcodes.html#CMP
// http://www.6502.org/users/obelisk/6502/reference.html#CMP
pub(crate) fn cmp(s: &mut VmState, operand: u8) {
    let result = s.reg.a.wrapping_sub(operand);
    set_flags_on_compare(s, result);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPX
// http://www.6502.org/users/obelisk/6502/reference.html#CPX
pub(crate) fn cpx(s: &mut VmState, operand: u8) {
    let result = s.reg.x.wrapping_sub(operand);
    set_flags_on_compare(s, result);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPY
// http://www.6502.org/users/obelisk/6502/reference.html#CPY
pub(crate) fn cpy(s: &mut VmState, operand: u8) {
    let result = s.reg.y.wrapping_sub(operand);
    set_flags_on_compare(s, result);
}
