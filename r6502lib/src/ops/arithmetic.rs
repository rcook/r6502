use crate::ops::helper::{is_carry, is_neg, is_overflow, is_zero};
use crate::{p_get, p_set, p_value, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#ADC
// http://www.6502.org/users/obelisk/6502/reference.html#ADC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
pub(crate) fn adc(s: &mut VmState, operand: u8) {
    fn nibbles(value: u8) -> (u8, u8) {
        (value >> 4, value & 0x0f)
    }

    fn from_nibbles(hi: u8, lo: u8) -> u8 {
        assert!(hi < 10 && lo < 10);
        (hi << 4) + lo
    }

    let lhs = s.reg.a;
    let rhs = operand;

    if p_get!(s.reg, D) {
        let (lhs_hi, lhs_lo) = nibbles(lhs);
        let (rhs_hi, rhs_lo) = nibbles(rhs);

        let t0 = lhs_lo + rhs_lo + p_value!(s.reg, C);
        let (t1, t2) = nibbles(t0);
        let (t3, result_lo) = (t2 / 10, t2 % 10);
        let carry = t1 + t3;

        let t5 = lhs_hi + rhs_hi + carry;
        let (carry, result_hi) = (t5 / 10, t5 % 10);

        let result = from_nibbles(result_hi, result_lo);

        s.reg.a = result;
        p_set!(s.reg, N, is_neg(result));
        //p_set!(s.reg, V, overflow); // TBD
        //p_set!(s.reg, Z, zero); // TBD
        p_set!(s.reg, C, carry != 0);
    } else {
        let value_word = lhs as u16 + rhs as u16 + p_value!(s.reg, C);
        let value = value_word as u8;

        let neg = is_neg(value);
        let overflow = is_overflow(lhs, rhs, value);
        let zero = is_zero(value);
        let carry = is_carry(value_word);

        s.reg.a = value;
        p_set!(s.reg, N, neg);
        p_set!(s.reg, V, overflow);
        p_set!(s.reg, Z, zero);
        p_set!(s.reg, C, carry);
    }
}

// http://www.6502.org/tutorials/6502opcodes.html#SBC
// http://www.6502.org/users/obelisk/6502/reference.html#SBC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
pub(crate) fn sbc(s: &mut VmState, operand: u8) {
    if p_get!(s.reg, D) {
        todo!("Decimal mode not implemented")
    }

    adc(s, !operand)
}

#[cfg(test)]
mod tests {
    use crate::ops::arithmetic::{adc, sbc};
    use crate::{reg, Reg, RegBuilder, VmBuilder, VmStateBuilder, P};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    // LDA #1; PHP; LDA #255; PLP; CLC; ADC #0
    #[case(reg!(0xff, 0x0000, N), reg!(0xff, 0x0000), 0x00)]
    // LDA #1; PHP; LDA #255; PLP; CLC; ADC #1
    #[case(reg!(0x00, 0x0000, Z, C), reg!(0xff, 0x0000), 0x01)]
    // LDA #1; PHP; LDA #255; PLP; SEC; ADC #0
    #[case(reg!(0x00, 0x0000, Z, C), reg!(0xff, 0x0000, C), 0x00)]
    // LDA #1; PHP; LDA #255; PLP; SEC; ADC #1
    #[case(reg!(0x01, 0x0000, C), reg!(0xff, 0x0000, C), 0x01)]
    // LDA #1; PHP; LDA #$12; PLP; CLC; ADC #$34
    #[case(reg!(0x46, 0x0000), reg!(0x12, 0x0000), 0x34)]
    fn adc_basics(#[case] expected_reg: Reg, #[case] reg: Reg, #[case] operand: u8) -> Result<()> {
        let mut s = VmStateBuilder::default().reg(reg).build()?;
        adc(&mut s, operand);
        assert_eq!(expected_reg, s.reg);
        Ok(())
    }

    #[rstest]
    #[case(reg!(0x46, 0x0000, D), reg!(0x12, 0x0000, D), 0x34)]
    #[case(reg!(0x51, 0x0000, D), reg!(0x12, 0x0000, D), 0x39)]
    fn adc_decimal_basics(
        #[case] expected_reg: Reg,
        #[case] reg: Reg,
        #[case] operand: u8,
    ) -> Result<()> {
        let mut s = VmStateBuilder::default().reg(reg).build()?;
        adc(&mut s, operand);
        assert_eq!(expected_reg, s.reg);
        Ok(())
    }

    #[test]
    fn foo() -> Result<()> {
        let reg = RegBuilder::default()
            .pc(50999)
            .s(33)
            .a(0x7f)
            .x(5)
            .y(99)
            .p(P::from_bits(0b10101010).expect("Must be valid"))
            .build()?;
        let mut s = VmStateBuilder::default().reg(reg).build()?;
        adc(&mut s, 0x87);
        assert_eq!(0x66, s.reg.a);
        assert_eq!(P::from_bits(0b00101011).expect("Must be valid"), s.reg.p);
        Ok(())
    }

    #[rstest]
    #[case(reg!(0xfe, 0x0000, N, C), reg!(0xff, 0x0000, C), 0x01)]
    fn sbc_basics(#[case] expected_reg: Reg, #[case] reg: Reg, #[case] operand: u8) -> Result<()> {
        let mut s = VmStateBuilder::default().reg(reg).build()?;
        sbc(&mut s, operand);
        assert_eq!(expected_reg, s.reg);
        Ok(())
    }
}
