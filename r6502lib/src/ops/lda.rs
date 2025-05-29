use crate::ops::helper::{is_neg, is_zero};
use crate::{set, Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#LDA
// http://www.6502.org/users/obelisk/6502/reference.html#LDA
pub(crate) fn lda(s: &mut VmState, operand: u8) -> Cycles {
    s.reg.a = operand;
    set!(s.reg, N, is_neg(operand));
    set!(s.reg, Z, is_zero(operand));
    2
}

#[cfg(test)]
mod tests {
    use crate::ops::lda::lda;
    use crate::{p, VmState, P};
    use rstest::rstest;

    #[rstest]
    // LDA #0
    #[case(p!(Z), 0x00)]
    // LDA #1
    #[case(p!(), 0x01)]
    // LDA #$255
    #[case(p!(N), 0xff)]
    fn basics(#[case] expected_p: P, #[case] operand: u8) {
        let mut s = VmState::default();
        s.reg.a = 0xff;
        let cycles = lda(&mut s, operand);
        assert_eq!(2, cycles);
        assert_eq!(operand, s.reg.a);
        assert_eq!(expected_p, s.reg.p);
    }
}
