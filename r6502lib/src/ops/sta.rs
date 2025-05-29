use crate::VmState;

// http://www.6502.org/tutorials/6502opcodes.html#STA
// http://www.6502.org/users/obelisk/6502/reference.html#STA
pub(crate) fn sta(s: &mut VmState, addr: u16) {
    s.memory[addr] = s.reg.a
}
