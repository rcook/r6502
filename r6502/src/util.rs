use crate::Cycles;

pub(crate) fn make_word(hi: u8, lo: u8) -> u16 {
    ((hi as u16) << 8) + lo as u16
}

pub(crate) fn split_word(value: u16) -> (u8, u8) {
    let hi = (value >> 8) as u8;
    let lo = value as u8;
    (hi, lo)
}

pub(crate) fn compute_branch(pc: u16, operand: u8) -> (u16, Cycles) {
    let lhs = pc as i32;

    // Treat operand as signed
    let rhs = (operand as i8) as i32;

    let result = (lhs + rhs) as u16;

    let cycles = 3; // TBD: Add 1 cycle if page boundary crossed
    (result, cycles)
}
