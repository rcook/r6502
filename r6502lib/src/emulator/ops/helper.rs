use crate::emulator::Cpu;
use crate::p_set;

pub const fn sign(value: u8) -> bool {
    (value & 0b1000_0000) != 0
}

pub const fn is_neg(value: u8) -> bool {
    sign(value)
}

pub const fn is_overflow(lhs: u8, rhs: u8, result: u8) -> bool {
    matches!(
        (sign(lhs), sign(rhs), sign(result)),
        (true, true, false) | (false, false, true)
    )
}

pub const fn is_zero(value: u8) -> bool {
    value == 0
}

pub const fn is_carry(value: u16) -> bool {
    (value & 0x0100) != 0
}

pub fn set_flags_on_value(cpu: &mut Cpu, operand: u8) {
    p_set!(cpu.reg, N, is_neg(operand));
    p_set!(cpu.reg, Z, is_zero(operand));
}
