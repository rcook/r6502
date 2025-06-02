use crate::ops::helper::{is_carry, is_neg, is_overflow, is_zero};
use crate::{p_get, p_set, p_value, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#ADC
// http://www.6502.org/users/obelisk/6502/reference.html#ADC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
pub(crate) fn adc(s: &mut VmState, operand: u8) {
    fn hi_nibble(value: u8) -> u8 {
        value >> 4
    }

    fn lo_nibble(value: u8) -> u8 {
        value & 0xf
    }

    fn is_bcd(value: u8) -> bool {
        hi_nibble(value) < 10 && lo_nibble(value) < 10
    }

    fn carry_nibble(value: u8) -> (u8, bool) {
        if value < 10 {
            (value, false)
        } else {
            (value - 10, true)
        }
    }

    let lhs = s.reg.a;
    let rhs = operand;

    if p_get!(s.reg, D) {
        // What do we do if operands are not valid BCD to start with?
        assert!(is_bcd(lhs) && is_bcd(rhs));

        let value_word = lhs as u16 + rhs as u16 + p_value!(s.reg, C);
        let value = value_word as u8;

        let (lo, carry) = carry_nibble(value & 0x0f);
        let (hi, _) = carry_nibble((value >> 4) + if carry { 1 } else { 0 });

        let new_value = (hi << 4) + lo;

        /*
        let neg = is_neg(value);
        let overflow = is_overflow(lhs, rhs, value);
        let zero = is_zero(value);
        let carry = is_carry(value_word);
        */

        s.reg.a = new_value;

        /*
        p_set!(s.reg, N, neg);
        p_set!(s.reg, V, overflow);
        p_set!(s.reg, Z, zero);
        p_set!(s.reg, C, carry);
        */
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
    use crate::{reg, Reg, VmStateBuilder};
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
    //#[case(reg!(0xff, 0x0000, N, D), reg!(0xff, 0x0000, D), 0x00)]
    //#[case(reg!(0x00, 0x0000, D, Z, C), reg!(0xff, 0x0000, D), 0x01)]
    //#[case(reg!(0x00, 0x0000, D, Z, C), reg!(0xff, 0x0000, D, C), 0x00)]
    //#[case(reg!(0x01, 0x0000, D, C), reg!(0xff, 0x0000, D, C), 0x01)]
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

    #[rstest]
    #[case(reg!(0xfe, 0x0000, N, C), reg!(0xff, 0x0000, C), 0x01)]
    fn sbc_basics(#[case] expected_reg: Reg, #[case] reg: Reg, #[case] operand: u8) -> Result<()> {
        let mut s = VmStateBuilder::default().reg(reg).build()?;
        sbc(&mut s, operand);
        assert_eq!(expected_reg, s.reg);
        Ok(())
    }
}
