use crate::{ops::helper::is_neg, set, Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#CMP
// http://www.6502.org/users/obelisk/6502/reference.html#CMP
pub(crate) fn cmp(s: &mut VmState, operand: u8) -> Cycles {
    let result = s.reg.a.wrapping_sub(operand);
    let neg = is_neg(result);
    set!(s.reg, N, neg);
    set!(s.reg, Z, result == 0);
    set!(s.reg, C, !neg);
    2
}
