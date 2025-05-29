use crate::ops::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#LDA
// http://www.6502.org/users/obelisk/6502/reference.html#LDA
pub(crate) fn lda(s: &mut VmState, operand: u8) -> Cycles {
    s.reg.a = operand;
    set_flags_on_value(s, operand);
    2
}

// http://www.6502.org/tutorials/6502opcodes.html#LDX
// http://www.6502.org/users/obelisk/6502/reference.html#LDX
pub(crate) fn ldx(s: &mut VmState, operand: u8) -> Cycles {
    s.reg.x = operand;
    set_flags_on_value(s, operand);
    2
}

// http://www.6502.org/tutorials/6502opcodes.html#LDY
// http://www.6502.org/users/obelisk/6502/reference.html#LDY
pub(crate) fn ldy(s: &mut VmState, operand: u8) -> Cycles {
    s.reg.y = operand;
    set_flags_on_value(s, operand);
    2
}

#[cfg(test)]
mod tests {
    use crate::ops::load::{lda, ldx, ldy};
    use crate::{p, VmState, P};
    use rstest::rstest;

    #[rstest]
    // LDA #0
    #[case(p!(Z), 0x00)]
    // LDA #1
    #[case(p!(), 0x01)]
    // LDA #$255
    #[case(p!(N), 0xff)]
    fn lda_basics(#[case] expected_p: P, #[case] operand: u8) {
        let mut s = VmState::default();
        s.reg.a = 0xff;
        let cycles = lda(&mut s, operand);
        assert_eq!(2, cycles);
        assert_eq!(operand, s.reg.a);
        assert_eq!(expected_p, s.reg.p);
    }

    #[rstest]
    // LDX #0
    #[case(p!(Z), 0x00)]
    // LDX #1
    #[case(p!(), 0x01)]
    // LDX #$255
    #[case(p!(N), 0xff)]
    fn ldx_basics(#[case] expected_p: P, #[case] operand: u8) {
        let mut s = VmState::default();
        s.reg.x = 0xff;
        let cycles = ldx(&mut s, operand);
        assert_eq!(2, cycles);
        assert_eq!(operand, s.reg.x);
        assert_eq!(expected_p, s.reg.p);
    }

    #[rstest]
    // LDY #0
    #[case(p!(Z), 0x00)]
    // LDY #1
    #[case(p!(), 0x01)]
    // LDY #$255
    #[case(p!(N), 0xff)]
    fn ldy_basics(#[case] expected_p: P, #[case] operand: u8) {
        let mut s = VmState::default();
        s.reg.y = 0xff;
        let cycles = ldy(&mut s, operand);
        assert_eq!(2, cycles);
        assert_eq!(operand, s.reg.y);
        assert_eq!(expected_p, s.reg.p);
    }
}
