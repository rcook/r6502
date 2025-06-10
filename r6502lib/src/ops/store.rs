use crate::ops::helper::set_flags_on_value;
use crate::Cpu;

// http://www.6502.org/tutorials/6502opcodes.html#DEC
// http://www.6502.org/users/obelisk/6502/reference.html#DEC
pub(crate) fn dec(state: &mut Cpu, addr: u16) {
    let result = state.memory.load(addr).wrapping_sub(1);
    state.memory.store(addr, result);
    set_flags_on_value(state, result);
}

// http://www.6502.org/tutorials/6502opcodes.html#INC
// http://www.6502.org/users/obelisk/6502/reference.html#INC
pub(crate) fn inc(state: &mut Cpu, addr: u16) {
    let result = state.memory.load(addr).wrapping_add(1);
    state.memory.store(addr, result);
    set_flags_on_value(state, result);
}

// http://www.6502.org/tutorials/6502opcodes.html#STA
// http://www.6502.org/users/obelisk/6502/reference.html#STA
pub(crate) fn sta(state: &mut Cpu, addr: u16) {
    state.memory.store(addr, state.reg.a)
}

// http://www.6502.org/tutorials/6502opcodes.html#STX
// http://www.6502.org/users/obelisk/6502/reference.html#STX
pub(crate) fn stx(state: &mut Cpu, addr: u16) {
    state.memory.store(addr, state.reg.x)
}

// http://www.6502.org/tutorials/6502opcodes.html#STY
// http://www.6502.org/users/obelisk/6502/reference.html#STY
pub(crate) fn sty(state: &mut Cpu, addr: u16) {
    state.memory.store(addr, state.reg.y)
}
