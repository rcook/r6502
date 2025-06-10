use crate::{p_set, Cpu};

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

pub(crate) fn set_flags_on_value(state: &mut Cpu, operand: u8) {
    p_set!(state.reg, N, is_neg(operand));
    p_set!(state.reg, Z, is_zero(operand));
}
