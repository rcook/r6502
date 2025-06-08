use crate::ops::helper::set_flags_on_value;
use crate::{p_set, CpuState};

// http://www.6502.org/tutorials/6502opcodes.html#AND
// http://www.6502.org/users/obelisk/6502/reference.html#AND
pub(crate) fn and(state: &mut CpuState, operand: u8) {
    state.reg.a &= operand;
    set_flags_on_value(state, state.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#BIT
// http://www.6502.org/users/obelisk/6502/reference.html#BIT
pub(crate) fn bit(state: &mut CpuState, operand: u8) {
    let value = state.reg.a & operand;
    p_set!(state.reg, N, (operand & 0b10000000) != 0);
    p_set!(state.reg, V, (operand & 0b01000000) != 0);
    p_set!(state.reg, Z, value == 0);
}

// http://www.6502.org/tutorials/6502opcodes.html#EOR
// http://www.6502.org/users/obelisk/6502/reference.html#EOR
pub(crate) fn eor(state: &mut CpuState, operand: u8) {
    state.reg.a ^= operand;
    set_flags_on_value(state, state.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#ORA
// http://www.6502.org/users/obelisk/6502/reference.html#ORA
pub(crate) fn ora(state: &mut CpuState, operand: u8) {
    state.reg.a |= operand;
    set_flags_on_value(state, state.reg.a);
}
