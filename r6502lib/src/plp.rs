use crate::{Cycles, VmState, P};

// http://www.6502.org/tutorials/6502opcodes.html#PLP
// http://www.6502.org/users/obelisk/6502/reference.html#PLP
pub(crate) fn plp(s: &mut VmState) -> Cycles {
    s.reg.p = P::from_bits(s.pull()).expect("Must be valid");
    4
}
