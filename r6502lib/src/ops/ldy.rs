use crate::ops::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#LDY
// http://www.6502.org/users/obelisk/6502/reference.html#LDY
pub(crate) fn ldy(s: &mut VmState, operand: u8) -> Cycles {
    s.reg.y = operand;
    set_flags_on_value(s, operand);
    2
}

#[cfg(test)]
mod tests {
    use crate::ops::ldy::ldy;
    use crate::{p, VmState, P};
    use rstest::rstest;

    #[rstest]
    // LDY #0
    #[case(p!(Z), 0x00)]
    // LDY #1
    #[case(p!(), 0x01)]
    // LDY #$255
    #[case(p!(N), 0xff)]
    fn basics(#[case] expected_p: P, #[case] operand: u8) {
        let mut s = VmState::default();
        s.reg.y = 0xff;
        let cycles = ldy(&mut s, operand);
        assert_eq!(2, cycles);
        assert_eq!(operand, s.reg.y);
        assert_eq!(expected_p, s.reg.p);
    }
}
