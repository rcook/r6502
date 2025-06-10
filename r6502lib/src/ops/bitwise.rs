use crate::ops::helper::set_flags_on_value;
use crate::{p_set, Cpu};

// http://www.6502.org/tutorials/6502opcodes.html#AND
// http://www.6502.org/users/obelisk/6502/reference.html#AND
pub(crate) fn and(cpu: &mut Cpu, operand: u8) {
    cpu.reg.a &= operand;
    set_flags_on_value(cpu, cpu.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#BIT
// http://www.6502.org/users/obelisk/6502/reference.html#BIT
pub(crate) fn bit(cpu: &mut Cpu, operand: u8) {
    let value = cpu.reg.a & operand;
    p_set!(cpu.reg, N, (operand & 0b10000000) != 0);
    p_set!(cpu.reg, V, (operand & 0b01000000) != 0);
    p_set!(cpu.reg, Z, value == 0);
}

// http://www.6502.org/tutorials/6502opcodes.html#EOR
// http://www.6502.org/users/obelisk/6502/reference.html#EOR
pub(crate) fn eor(cpu: &mut Cpu, operand: u8) {
    cpu.reg.a ^= operand;
    set_flags_on_value(cpu, cpu.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#ORA
// http://www.6502.org/users/obelisk/6502/reference.html#ORA
pub(crate) fn ora(cpu: &mut Cpu, operand: u8) {
    cpu.reg.a |= operand;
    set_flags_on_value(cpu, cpu.reg.a);
}
