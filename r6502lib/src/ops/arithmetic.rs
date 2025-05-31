use crate::ops::helper::{is_carry, is_neg, is_overflow, is_zero};
use crate::{p_get, p_value, p_set, Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#ADC
// http://www.6502.org/users/obelisk/6502/reference.html#ADC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
pub(crate) fn adc(s: &mut VmState, operand: u8) -> Cycles {
    if p_get!(s.reg, D) {
        todo!("Decimal mode not implemented")
    }

    let lhs = s.reg.a;
    let rhs = operand;

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
    2
}

// http://www.6502.org/tutorials/6502opcodes.html#SBC
// http://www.6502.org/users/obelisk/6502/reference.html#SBC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
pub(crate) fn sbc(s: &mut VmState, operand: u8) -> Cycles {
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
        let cycles = adc(&mut s, operand);
        assert_eq!(2, cycles);
        assert_eq!(expected_reg, s.reg);
        Ok(())
    }

    #[rstest]
    #[case(reg!(0xfe, 0x0000, N, C), reg!(0xff, 0x0000, C), 0x01)]
    fn sbc_basics(#[case] expected_reg: Reg, #[case] reg: Reg, #[case] operand: u8) -> Result<()> {
        let mut s = VmStateBuilder::default().reg(reg).build()?;
        let cycles = sbc(&mut s, operand);
        assert_eq!(2, cycles);
        assert_eq!(expected_reg, s.reg);
        Ok(())
    }
}
