use crate::ops::helper::set_flags_on_value;
use crate::VmState;

// http://www.6502.org/tutorials/6502opcodes.html#DEC
// http://www.6502.org/users/obelisk/6502/reference.html#DEC
pub(crate) fn dec(s: &mut VmState, addr: u16) {
    let result = s.memory[addr].wrapping_sub(1);
    s.memory[addr] = result;
    set_flags_on_value(s, result);
}

// http://www.6502.org/tutorials/6502opcodes.html#INC
// http://www.6502.org/users/obelisk/6502/reference.html#INC
pub(crate) fn inc(s: &mut VmState, addr: u16) {
    let result = s.memory[addr].wrapping_add(1);
    s.memory[addr] = result;
    set_flags_on_value(s, result);
}

// http://www.6502.org/tutorials/6502opcodes.html#STA
// http://www.6502.org/users/obelisk/6502/reference.html#STA
pub(crate) fn sta(s: &mut VmState, addr: u16) {
    s.memory[addr] = s.reg.a
}

// http://www.6502.org/tutorials/6502opcodes.html#STX
// http://www.6502.org/users/obelisk/6502/reference.html#STX
pub(crate) fn stx(s: &mut VmState, addr: u16) {
    s.memory[addr] = s.reg.x
}

// http://www.6502.org/tutorials/6502opcodes.html#STY
// http://www.6502.org/users/obelisk/6502/reference.html#STY
pub(crate) fn sty(s: &mut VmState, addr: u16) {
    s.memory[addr] = s.reg.y
}
