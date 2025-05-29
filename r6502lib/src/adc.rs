use crate::{get, set, value, Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#ADC
// http://www.6502.org/users/obelisk/6502/reference.html#ADC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
pub(crate) fn adc(s: &mut VmState, operand: u8) -> Cycles {
    fn sign(value: u8) -> bool {
        (value & 0b10000000) != 0
    }

    fn is_neg(value: u8) -> bool {
        sign(value)
    }

    fn is_overflow(lhs: u8, rhs: u8, result: u8) -> bool {
        matches!(
            (sign(lhs), sign(rhs), sign(result)),
            (true, true, false) | (false, false, true)
        )
    }

    fn is_zero(value: u8) -> bool {
        value == 0
    }

    fn is_carry(value: u16) -> bool {
        (value & 0x0100) != 0
    }

    if get!(s.reg, D) {
        todo!("Decimal mode not implemented")
    }

    let lhs = s.reg.a;
    let rhs = operand;

    let result_word = lhs as u16 + rhs as u16 + value!(s.reg, C);
    let result = result_word as u8;

    let neg = is_neg(result);
    let overflow = is_overflow(lhs, rhs, result);
    let zero = is_zero(result);
    let carry = is_carry(result_word);

    s.reg.a = result;
    set!(s.reg, N, neg);
    set!(s.reg, V, overflow);
    set!(s.reg, Z, zero);
    set!(s.reg, C, carry);
    2
}

#[cfg(test)]
mod tests {
    use crate::{adc, reg, Memory, Reg, VmState};
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
    fn basics(#[case] expected_reg: Reg, #[case] reg: Reg, #[case] operand: u8) {
        let mut s = VmState {
            reg,
            memory: Memory::new(),
        };
        let cycles = adc(&mut s, operand);
        assert_eq!(2, cycles);
        assert_eq!(expected_reg, s.reg);
    }
}
