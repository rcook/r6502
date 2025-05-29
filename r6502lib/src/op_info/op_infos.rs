pub(crate) use items::*;

mod absolute {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                _ = $crate::ops::$f::$f(s, s.memory[addr]);
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                $crate::ops::$f::$f(s, addr);
                $cycles
            }
        };
    }

    wrap!(adc, 4);
    wrap!(cmp, 4);
    wrap!(lda, 4);
    wrap!(ldx, 4);
    wrap!(ldy, 4);
    wrap_store!(sta, 4);
}

mod absolute_x {
    macro_rules! wrap {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                _ = $crate::ops::$f::$f(s, s.memory[addr.wrapping_add(s.reg.x as u16)]);
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                $crate::ops::$f::$f(s, addr.wrapping_add(s.reg.x as u16));
                $cycles
            }
        };
    }

    wrap!(cmp, 4, 5);
    wrap!(lda, 4, 5);
    wrap_store!(sta, 5, 5);
}

mod absolute_y {
    #[allow(unused)]
    macro_rules! wrap {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                _ = $crate::ops::$f::$f(s, s.memory[addr.wrapping_add(s.reg.y as u16)]);
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                $crate::ops::$f::$f(s, addr.wrapping_add(s.reg.y as u16));
                $cycles
            }
        };
    }

    wrap_store!(sta, 5, 5);
}

mod indirect_indexed_y {
    macro_rules! wrap {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u8) -> $crate::Cycles {
                let addr = (s.memory[addr as u16] as u16).wrapping_add(s.reg.y as u16);
                _ = $crate::ops::$f::$f(s, s.memory[addr]);
                $cycles
            }
        };
    }

    wrap!(lda, 5, 6);
}

mod zero_page {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u8) -> $crate::Cycles {
                _ = $crate::ops::$f::$f(s, s.memory[addr as u16]);
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u8) -> $crate::Cycles {
                $crate::ops::$f::$f(s, addr as u16);
                $cycles
            }
        };
    }

    wrap!(adc, 3);
    wrap!(cmp, 3);
    wrap!(lda, 3);
    wrap!(ldx, 3);
    wrap!(ldy, 3);
    wrap_store!(sta, 3);
}

#[iter_mod::make_items]
mod items {
    use crate::OpInfo;

    macro_rules! absolute {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Absolute,
                op: $crate::Op::Word($crate::WordOp::new($crate::ops::$f::$f)),
            }
        };
    }

    macro_rules! immediate {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Immediate,
                op: $crate::Op::Byte($crate::ByteOp::new($crate::ops::$f::$f)),
            }
        };
    }

    macro_rules! implied {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Implied,
                op: $crate::Op::NoOperand($crate::NoOperandOp::new($crate::ops::$f::$f)),
            }
        };
    }

    macro_rules! relative {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Relative,
                op: $crate::Op::Byte($crate::ByteOp::new($crate::ops::$f::$f)),
            }
        };
    }

    macro_rules! absolute_wrapped {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Absolute,
                op: $crate::Op::Word($crate::WordOp::new($crate::op_info::op_infos::absolute::$f)),
            }
        };
    }

    macro_rules! absolute_x_wrapped {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::AbsoluteX,
                op: $crate::Op::Word($crate::WordOp::new(
                    $crate::op_info::op_infos::absolute_x::$f,
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
                    $crate::op_info::op_infos::absolute_y::$f,
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
                    $crate::op_info::op_infos::indirect_indexed_y::$f,
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
                    $crate::op_info::op_infos::zero_page::$f,
                )),
            }
        };
    }

    pub(crate) const ADC_ABS: OpInfo = absolute_wrapped!(AdcAbs, adc);
    pub(crate) const ADC_IMM: OpInfo = immediate!(AdcImm, adc);
    pub(crate) const ADC_ZP: OpInfo = zero_page_wrapped!(AdcZp, adc);
    pub(crate) const BCC: OpInfo = relative!(Bcc, bcc);
    pub(crate) const BCS: OpInfo = relative!(Bcs, bcs);
    pub(crate) const BEQ: OpInfo = relative!(Beq, beq);
    pub(crate) const BMI: OpInfo = relative!(Bmi, bmi);
    pub(crate) const BNE: OpInfo = relative!(Bne, bne);
    pub(crate) const BPL: OpInfo = relative!(Bpl, bpl);
    pub(crate) const BRK: OpInfo = implied!(Brk, brk);
    pub(crate) const BVC: OpInfo = relative!(Bvc, bvc);
    pub(crate) const BVS: OpInfo = relative!(Bvs, bvs);
    pub(crate) const CMP_ABS: OpInfo = absolute_wrapped!(CmpAbs, cmp);
    pub(crate) const CMP_ABS_X: OpInfo = absolute_x_wrapped!(CmpAbsX, cmp);
    pub(crate) const CMP_IMM: OpInfo = immediate!(CmpImm, cmp);
    pub(crate) const CMP_ZP: OpInfo = zero_page_wrapped!(CmpZp, cmp);
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
    pub(crate) const STA_ABS: OpInfo = absolute_wrapped!(StaAbs, sta);
    pub(crate) const STA_ABS_X: OpInfo = absolute_x_wrapped!(StaAbsX, sta);
    pub(crate) const STA_ABS_Y: OpInfo = absolute_y_wrapped!(StaAbsY, sta);
    pub(crate) const STA_ZP: OpInfo = zero_page_wrapped!(StaZp, sta);
}
