use crate::emulator::ops::helper::set_flags_on_value;
use crate::emulator::Cpu;
use crate::p_set;

// http://www.6502.org/tutorials/6502opcodes.html#AND
// http://www.6502.org/users/obelisk/6502/reference.html#AND
pub fn and(cpu: &mut Cpu, operand: u8) {
    cpu.reg.a &= operand;
    set_flags_on_value(cpu, cpu.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#BIT
// http://www.6502.org/users/obelisk/6502/reference.html#BIT
pub fn bit(cpu: &mut Cpu, operand: u8) {
    let value = cpu.reg.a & operand;
    p_set!(cpu.reg, N, (operand & 0b1000_0000) != 0);
    p_set!(cpu.reg, V, (operand & 0b0100_0000) != 0);
    p_set!(cpu.reg, Z, value == 0);
}

// http://www.6502.org/tutorials/6502opcodes.html#EOR
// http://www.6502.org/users/obelisk/6502/reference.html#EOR
pub fn eor(cpu: &mut Cpu, operand: u8) {
    cpu.reg.a ^= operand;
    set_flags_on_value(cpu, cpu.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#ORA
// http://www.6502.org/users/obelisk/6502/reference.html#ORA
pub fn ora(cpu: &mut Cpu, operand: u8) {
    cpu.reg.a |= operand;
    set_flags_on_value(cpu, cpu.reg.a);
}
