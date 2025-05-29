use crate::ops::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#LDX
// http://www.6502.org/users/obelisk/6502/reference.html#LDX
pub(crate) fn ldx(s: &mut VmState, operand: u8) -> Cycles {
    s.reg.x = operand;
    set_flags_on_value(s, operand);
    2
}

#[cfg(test)]
mod tests {
    use crate::ops::ldx::ldx;
    use crate::{p, VmState, P};
    use rstest::rstest;

    #[rstest]
    // LDX #0
    #[case(p!(Z), 0x00)]
    // LDX #1
    #[case(p!(), 0x01)]
    // LDX #$255
    #[case(p!(N), 0xff)]
    fn basics(#[case] expected_p: P, #[case] operand: u8) {
        let mut s = VmState::default();
        s.reg.x = 0xff;
        let cycles = ldx(&mut s, operand);
        assert_eq!(2, cycles);
        assert_eq!(operand, s.reg.x);
        assert_eq!(expected_p, s.reg.p);
    }
}
