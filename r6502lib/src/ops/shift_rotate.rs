use crate::ops::helper::{set_flags_on_value, sign};
use crate::{p_get, p_set, OpCycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#ASL
// http://www.6502.org/users/obelisk/6502/reference.html#ASL
pub(crate) fn asl_acc(s: &mut VmState) -> OpCycles {
    s.reg.a = asl_helper(s, s.reg.a);
    2
}

// http://www.6502.org/tutorials/6502opcodes.html#ASL
// http://www.6502.org/users/obelisk/6502/reference.html#ASL
pub(crate) fn asl(s: &mut VmState, addr: u16) -> OpCycles {
    s.memory[addr] = asl_helper(s, s.memory[addr]);
    6
}

// http://www.6502.org/tutorials/6502opcodes.html#LSR
// http://www.6502.org/users/obelisk/6502/reference.html#LSR
pub(crate) fn lsr_acc(s: &mut VmState) -> OpCycles {
    s.reg.a = lsr_helper(s, s.reg.a);
    2
}

// http://www.6502.org/tutorials/6502opcodes.html#LSR
// http://www.6502.org/users/obelisk/6502/reference.html#LSR
pub(crate) fn lsr(s: &mut VmState, addr: u16) -> OpCycles {
    s.memory[addr] = lsr_helper(s, s.memory[addr]);
    6
}

// http://www.6502.org/tutorials/6502opcodes.html#ROL
// http://www.6502.org/users/obelisk/6502/reference.html#ROL
pub(crate) fn rol_acc(s: &mut VmState) -> OpCycles {
    s.reg.a = lsr_helper(s, s.reg.a);
    2
}

// http://www.6502.org/tutorials/6502opcodes.html#ROL
// http://www.6502.org/users/obelisk/6502/reference.html#ROL
pub(crate) fn rol(s: &mut VmState, addr: u16) -> OpCycles {
    s.memory[addr] = rol_helper(s, s.memory[addr]);
    6
}

// http://www.6502.org/tutorials/6502opcodes.html#ROR
// http://www.6502.org/users/obelisk/6502/reference.html#ROR
pub(crate) fn ror_acc(s: &mut VmState) -> OpCycles {
    s.reg.a = ror_helper(s, s.reg.a);
    2
}

// http://www.6502.org/tutorials/6502opcodes.html#ROR
// http://www.6502.org/users/obelisk/6502/reference.html#ROR
pub(crate) fn ror(s: &mut VmState, addr: u16) -> OpCycles {
    s.memory[addr] = ror_helper(s, s.memory[addr]);
    6
}

fn asl_helper(s: &mut VmState, operand: u8) -> u8 {
    p_set!(s.reg, C, sign(operand));
    let new_value = operand << 1;
    set_flags_on_value(s, new_value);
    new_value
}

fn lsr_helper(s: &mut VmState, operand: u8) -> u8 {
    p_set!(s.reg, C, (operand & 0x01) != 0);
    let new_value = operand >> 1;
    set_flags_on_value(s, new_value);
    new_value
}

fn rol_helper(s: &mut VmState, operand: u8) -> u8 {
    let old_carry = p_get!(s.reg, C);
    p_set!(s.reg, C, sign(operand));
    let new_value = (operand << 1) | (if old_carry { 0x01 } else { 0x00 });
    set_flags_on_value(s, new_value);
    new_value
}

fn ror_helper(s: &mut VmState, operand: u8) -> u8 {
    let old_carry = p_get!(s.reg, C);
    p_set!(s.reg, C, (operand & 0x01) != 0);
    let new_value = (operand >> 1) | (if old_carry { 0x80 } else { 0x00 });
    set_flags_on_value(s, new_value);
    new_value
}
