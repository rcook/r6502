use crate::ops::helper::{is_carry, is_neg, is_overflow, is_zero};
use crate::{p_get, p_set, p_value, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#ADC
// http://www.6502.org/users/obelisk/6502/reference.html#ADC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
pub(crate) fn adc(s: &mut VmState, operand: u8) {
    if p_get!(s.reg, D) {
        let lhs = s.reg.a;
        let rhs = operand;

        println!("p(before) = {p} {p:08b}", p = s.reg.p.bits());
        println!("a         = {a} {a:02X}", a = lhs);
        println!("value     = {value} {value:02X}", value = rhs);

        let temp_result = lhs.wrapping_add(rhs).wrapping_add(p_value!(s.reg, C));

        let (lhs_hi, lhs_lo) = (lhs >> 4, lhs & 0xf);
        let (rhs_hi, rhs_lo) = (rhs >> 4, rhs & 0xf);

        let t0 = lhs_lo + rhs_lo + p_value!(s.reg, C);
        let t1 = if t0 < 10 { t0 } else { t0 + 6 };
        let result_lo = t1 & 0x0f;
        let carry = t1 >> 4;

        let temp_result = (temp_result & 0xf0) + result_lo;
        p_set!(s.reg, N, is_neg(temp_result));

        let t2 = lhs_hi + rhs_hi + carry;
        let t3 = if t2 < 10 { t2 } else { t2 + 6 };
        let result_hi = t3 & 0x0f;
        let carry = t3 >> 4;

        let result = (result_hi << 4) + result_lo;

        s.reg.a = result;
        p_set!(s.reg, V, is_overflow(lhs, rhs, result));
        p_set!(s.reg, Z, is_zero(result));
        p_set!(s.reg, C, carry != 0);

        println!("p(after)  = {p} {p:08b}", p = s.reg.p.bits());
    } else {
        let lhs = s.reg.a;
        let rhs = operand;

        println!("p(before) = {p} {p:08b}", p = s.reg.p.bits());
        println!("a         = {a} {a:02X}", a = lhs);
        println!("value     = {value} {value:02X}", value = rhs);

        let result_word = lhs as u16 + rhs as u16 + p_value!(s.reg, C);
        let result = result_word as u8;

        s.reg.a = result;
        p_set!(s.reg, N, is_neg(result));
        p_set!(s.reg, V, is_overflow(lhs, rhs, result));
        p_set!(s.reg, Z, is_zero(result));
        p_set!(s.reg, C, is_carry(result_word));

        println!("p(after)  = {p} {p:08b}", p = s.reg.p.bits());
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
    use crate::{reg, Reg, RegBuilder, VmStateBuilder, P};
    use anyhow::Result;
    use rstest::rstest;

    macro_rules! _p {
        ($value: expr) => {
            $crate::P::from_bits($value).unwrap()
        };
    }

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
    // cargo run -p r6502validation -- run-json '{ "name": "61 61 a2", "initial": { "pc": 31864, "s": 184, "a": 245, "x": 248, "y": 67, "p": 44, "ram": [ [31864, 97], [31865, 97], [31866, 162], [97, 187], [89, 99], [90, 228], [58467, 109]]}, "final": { "pc": 31866, "s": 184, "a": 200, "x": 248, "y": 67, "p": 45, "ram": [ [89, 99], [90, 228], [97, 187], [31864, 97], [31865, 97], [31866, 162], [58467, 109]]}, "cycles": [ [31864, 97, "read"], [31865, 97, "read"], [97, 187, "read"], [89, 99, "read"], [90, 228, "read"], [58467, 109, "read"]] }'
    #[case(_p!(45), 200, _p!(44), 245, 109)]
    // cargo run -p r6502validation -- run-json '{ "name": "61 0f 6b", "initial": { "pc": 28568, "s": 191, "a": 136, "x": 3, "y": 89, "p": 175, "ram": [ [28568, 97], [28569, 15], [28570, 107], [15, 179], [18, 143], [19, 69], [17807, 106]]}, "final": { "pc": 28570, "s": 191, "a": 89, "x": 3, "y": 89, "p": 173, "ram": [ [15, 179], [18, 143], [19, 69], [17807, 106], [28568, 97], [28569, 15], [28570, 107]]}, "cycles": [ [28568, 97, "read"], [28569, 15, "read"], [15, 179, "read"], [18, 143, "read"], [19, 69, "read"], [17807, 106, "read"]] }'
    #[case(_p!(173), 89, _p!(175), 136, 106)]
    // cargo run -p r6502validation -- run-json '{ "name": "61 7b 59", "initial": { "pc": 12139, "s": 205, "a": 80, "x": 208, "y": 251, "p": 235, "ram": [ [12139, 97], [12140, 123], [12141, 89], [123, 219], [75, 208], [76, 222], [57040, 4]]}, "final": { "pc": 12141, "s": 205, "a": 85, "x": 208, "y": 251, "p": 40, "ram": [ [75, 208], [76, 222], [123, 219], [12139, 97], [12140, 123], [12141, 89], [57040, 4]]}, "cycles": [ [12139, 97, "read"], [12140, 123, "read"], [123, 219, "read"], [75, 208, "read"], [76, 222, "read"], [57040, 4, "read"]] }'
    #[case(_p!(40), 85, _p!(235), 80, 4)]
    fn adc_decimal_basics(
        #[case] expected_p: P,
        #[case] expected_a: u8,
        #[case] p: P,
        #[case] a: u8,
        #[case] value: u8,
    ) -> Result<()> {
        let reg = RegBuilder::default().a(a).p(p).build()?;
        let mut s = VmStateBuilder::default().reg(reg).build()?;
        adc(&mut s, value);
        assert_eq!(expected_a, s.reg.a);
        assert_eq!(expected_p, s.reg.p);
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
