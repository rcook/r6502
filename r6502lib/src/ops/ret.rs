use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#RTS
// http://www.6502.org/users/obelisk/6502/reference.html#RTS
pub(crate) fn rts(s: &mut VmState) -> Cycles {
    let return_addr = s.pull_word() + 1;
    s.reg.pc = return_addr;
    6
}
