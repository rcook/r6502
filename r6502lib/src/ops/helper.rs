use crate::{set, Cycles, VmState, P};

pub(crate) fn sign(value: u8) -> bool {
    (value & 0b10000000) != 0
}

pub(crate) fn is_neg(value: u8) -> bool {
    sign(value)
}

pub(crate) fn is_overflow(lhs: u8, rhs: u8, result: u8) -> bool {
    matches!(
        (sign(lhs), sign(rhs), sign(result)),
        (true, true, false) | (false, false, true)
    )
}

pub(crate) fn is_zero(value: u8) -> bool {
    value == 0
}

pub(crate) fn is_carry(value: u16) -> bool {
    (value & 0x0100) != 0
}

pub(crate) fn set_flags_on_value(s: &mut VmState, operand: u8) {
    set!(s.reg, N, is_neg(operand));
    set!(s.reg, Z, is_zero(operand));
}

pub(crate) fn set_flags_on_compare(s: &mut VmState, operand: u8) {
    let neg = is_neg(operand);
    set!(s.reg, N, neg);
    set!(s.reg, Z, operand == 0);
    set!(s.reg, C, !neg);
}

pub(crate) fn branch(s: &mut VmState, operand: u8, p: P, flag_value: bool) -> Cycles {
    if s.reg.p.contains(p) == flag_value {
        // Sign-extend the operand before adding it
        let new_pc = s.reg.pc.wrapping_add((operand as i8) as u16);

        let current_page = s.reg.pc >> 8;
        let new_page = new_pc >> 8;

        s.reg.pc = new_pc;

        if new_page == current_page {
            3
        } else {
            4
        }
    } else {
        2
    }
}
