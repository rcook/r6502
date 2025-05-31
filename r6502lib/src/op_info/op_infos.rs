pub(crate) use items::*;

#[iter_mod::make_items]
mod items {
    use crate::OpInfo;

    macro_rules! absolute {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Absolute,
                op: $crate::Op::Word($crate::WordOp::new($crate::ops::$f)),
            }
        };
    }

    macro_rules! immediate {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Immediate,
                op: $crate::Op::Byte($crate::ByteOp::new($crate::ops::$f)),
            }
        };
    }

    macro_rules! implied {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Implied,
                op: $crate::Op::NoOperand($crate::NoOperandOp::new($crate::ops::$f)),
            }
        };
    }

    macro_rules! relative {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Relative,
                op: $crate::Op::Byte($crate::ByteOp::new($crate::ops::$f)),
            }
        };
    }

    macro_rules! absolute_wrapped {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Absolute,
                op: $crate::Op::Word($crate::WordOp::new($crate::op_info::wrappers::absolute::$f)),
            }
        };
    }

    macro_rules! absolute_x_wrapped {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::AbsoluteX,
                op: $crate::Op::Word($crate::WordOp::new(
                    $crate::op_info::wrappers::absolute_x::$f,
                )),
            }
        };
    }

    macro_rules! absolute_y_wrapped {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::AbsoluteY,
                op: $crate::Op::Word($crate::WordOp::new(
                    $crate::op_info::wrappers::absolute_y::$f,
                )),
            }
        };
    }

    macro_rules! indexed_indirect_x_wrapped {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::IndexedIndirectX,
                op: $crate::Op::Byte($crate::ByteOp::new(
                    $crate::op_info::wrappers::indexed_indirect_x::$f,
                )),
            }
        };
    }

    macro_rules! indirect_indexed_y_wrapped {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::IndirectIndexedY,
                op: $crate::Op::Byte($crate::ByteOp::new(
                    $crate::op_info::wrappers::indirect_indexed_y::$f,
                )),
            }
        };
    }

    macro_rules! zero_page_wrapped {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::ZeroPage,
                op: $crate::Op::Byte($crate::ByteOp::new(
                    $crate::op_info::wrappers::zero_page::$f,
                )),
            }
        };
    }

    macro_rules! zero_page_x_wrapped {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::ZeroPageX,
                op: $crate::Op::Byte($crate::ByteOp::new(
                    $crate::op_info::wrappers::zero_page_x::$f,
                )),
            }
        };
    }

    pub(crate) const ADC_ABS: OpInfo = absolute_wrapped!(AdcAbs, adc);
    pub(crate) const ADC_ABS_X: OpInfo = absolute_x_wrapped!(AdcAbsX, adc);
    pub(crate) const ADC_ABS_Y: OpInfo = absolute_y_wrapped!(AdcAbsY, adc);
    pub(crate) const ADC_IMM: OpInfo = immediate!(AdcImm, adc);
    pub(crate) const ADC_IND_X: OpInfo = indexed_indirect_x_wrapped!(AdcIndX, adc);
    pub(crate) const ADC_IND_Y: OpInfo = indirect_indexed_y_wrapped!(AdcIndY, adc);
    pub(crate) const ADC_ZP: OpInfo = zero_page_wrapped!(AdcZp, adc);
    pub(crate) const ADC_ZP_X: OpInfo = zero_page_x_wrapped!(AdcZpX, adc);
    pub(crate) const BCC: OpInfo = relative!(Bcc, bcc);
    pub(crate) const BCS: OpInfo = relative!(Bcs, bcs);
    pub(crate) const BEQ: OpInfo = relative!(Beq, beq);
    pub(crate) const BMI: OpInfo = relative!(Bmi, bmi);
    pub(crate) const BNE: OpInfo = relative!(Bne, bne);
    pub(crate) const BPL: OpInfo = relative!(Bpl, bpl);
    pub(crate) const BRK: OpInfo = implied!(Brk, brk);
    pub(crate) const BVC: OpInfo = relative!(Bvc, bvc);
    pub(crate) const BVS: OpInfo = relative!(Bvs, bvs);
    pub(crate) const CLC: OpInfo = implied!(Clc, clc);
    pub(crate) const CLD: OpInfo = implied!(Cld, cld);
    pub(crate) const CLI: OpInfo = implied!(Cli, cli);
    pub(crate) const CLV: OpInfo = implied!(Clv, clv);
    pub(crate) const CMP_ABS: OpInfo = absolute_wrapped!(CmpAbs, cmp);
    pub(crate) const CMP_ABS_X: OpInfo = absolute_x_wrapped!(CmpAbsX, cmp);
    pub(crate) const CMP_IMM: OpInfo = immediate!(CmpImm, cmp);
    pub(crate) const CMP_ZP: OpInfo = zero_page_wrapped!(CmpZp, cmp);
    pub(crate) const CPX_ABS: OpInfo = absolute_wrapped!(CpxAbs, cpx);
    pub(crate) const CPX_IMM: OpInfo = immediate!(CpxImm, cpx);
    pub(crate) const CPX_ZP: OpInfo = zero_page_wrapped!(CpxZp, cpx);
    pub(crate) const CPY_ABS: OpInfo = absolute_wrapped!(CpyAbs, cpy);
    pub(crate) const CPY_IMM: OpInfo = immediate!(CpyImm, cpy);
    pub(crate) const CPY_ZP: OpInfo = zero_page_wrapped!(CpyZp, cpy);
    pub(crate) const DEX: OpInfo = implied!(Dex, dex);
    pub(crate) const DEY: OpInfo = implied!(Dey, dey);
    pub(crate) const INX: OpInfo = implied!(Inx, inx);
    pub(crate) const INY: OpInfo = implied!(Iny, iny);
    pub(crate) const JMP_ABS: OpInfo = absolute!(JmpAbs, jmp);
    pub(crate) const JSR: OpInfo = absolute!(Jsr, jsr);
    pub(crate) const LDA_ABS: OpInfo = absolute_wrapped!(LdaAbs, lda);
    pub(crate) const LDA_ABS_X: OpInfo = absolute_x_wrapped!(LdaAbsX, lda);
    pub(crate) const LDA_IMM: OpInfo = immediate!(LdaImm, lda);
    pub(crate) const LDA_IND_Y: OpInfo = indirect_indexed_y_wrapped!(LdaIndY, lda);
    pub(crate) const LDA_ZP: OpInfo = zero_page_wrapped!(LdaZp, lda);
    pub(crate) const LDX_ABS: OpInfo = absolute_wrapped!(LdxAbs, ldx);
    pub(crate) const LDX_IMM: OpInfo = immediate!(LdxImm, ldx);
    pub(crate) const LDX_ZP: OpInfo = zero_page_wrapped!(LdxZp, ldx);
    pub(crate) const LDY_ABS: OpInfo = absolute_wrapped!(LdyAbs, ldy);
    pub(crate) const LDY_IMM: OpInfo = immediate!(LdyImm, ldy);
    pub(crate) const LDY_ZP: OpInfo = zero_page_wrapped!(LdyZp, ldy);
    pub(crate) const NOP: OpInfo = implied!(Nop, nop);
    pub(crate) const PHA: OpInfo = implied!(Pha, pha);
    pub(crate) const PHP: OpInfo = implied!(Php, php);
    pub(crate) const PLA: OpInfo = implied!(Pla, pla);
    pub(crate) const PLP: OpInfo = implied!(Plp, plp);
    pub(crate) const RTS: OpInfo = implied!(Rts, rts);
    pub(crate) const SEC: OpInfo = implied!(Sec, sec);
    pub(crate) const SED: OpInfo = implied!(Sed, sed);
    pub(crate) const SEI: OpInfo = implied!(Sei, sei);
    pub(crate) const STA_ABS: OpInfo = absolute_wrapped!(StaAbs, sta);
    pub(crate) const STA_ABS_X: OpInfo = absolute_x_wrapped!(StaAbsX, sta);
    pub(crate) const STA_ABS_Y: OpInfo = absolute_y_wrapped!(StaAbsY, sta);
    pub(crate) const STA_ZP: OpInfo = zero_page_wrapped!(StaZp, sta);
    pub(crate) const TAX: OpInfo = implied!(Tax, tax);
    pub(crate) const TAY: OpInfo = implied!(Tay, tay);
    pub(crate) const TXA: OpInfo = implied!(Txa, txa);
    pub(crate) const TYA: OpInfo = implied!(Tya, tya);
}
