use crate::ops::helper::branch;
use crate::{Cycles, VmState, P};

// http://www.6502.org/tutorials/6502opcodes.html#BEQ
// http://www.6502.org/users/obelisk/6502/reference.html#BEQ
pub(crate) fn beq(s: &mut VmState, operand: u8) -> Cycles {
    branch(s, operand, P::Z, true)
}

#[cfg(test)]
mod tests {
    use crate::ops::beq::beq;
    use crate::{set, Cycles, VmState};
    use rstest::rstest;

    #[rstest]
    #[case(2, 0x1000, false, 0x1000, 0x10)]
    #[case(3, 0x1010, true, 0x1000, 0x10)]
    #[case(3, 0x10e0, true, 0x10f0, 0xf0)]
    #[case(4, 0x0ff0, true, 0x1000, 0xf0)]
    fn basics(
        #[case] expected_cycles: Cycles,
        #[case] expected_pc: u16,
        #[case] flag_value: bool,
        #[case] pc: u16,
        #[case] operand: u8,
    ) {
        let mut s = VmState::default();
        set!(s.reg, Z, flag_value);
        s.reg.pc = pc;
        let cycles = beq(&mut s, operand);
        assert_eq!(expected_cycles, cycles);
        assert_eq!(expected_pc, s.reg.pc);
    }
}
