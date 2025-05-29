pub(crate) use items::*;

mod absolute {
    macro_rules! wrap {
        ($f: ident, $extra_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, value: u16) -> $crate::Cycles {
                $crate::$f(s, s.memory[value]) + $extra_cycles
            }
        };
    }

    wrap!(adc, 2);
}

mod zero_page {
    macro_rules! wrap {
        ($f: ident, $extra_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, value: u8) -> $crate::Cycles {
                $crate::$f(s, s.memory[value as u16]) + $extra_cycles
            }
        };
    }

    wrap!(adc, 1);
}

#[iter_mod::make_items]
mod items {
    use crate::OpInfo;

    macro_rules! absolute {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Absolute,
                op: $crate::Op::Word($crate::WordOp::new($crate::$f)),
            }
        };
    }

    macro_rules! immediate {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Immediate,
                op: $crate::Op::Byte($crate::ByteOp::new($crate::$f)),
            }
        };
    }

    macro_rules! implied {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Implied,
                op: $crate::Op::NoOperand($crate::NoOperandOp::new($crate::$f)),
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
    pub(crate) const BRK: OpInfo = implied!(Brk, brk);
    pub(crate) const JMP_ABS: OpInfo = absolute!(JmpAbs, jmp);
    pub(crate) const JSR: OpInfo = absolute!(Jsr, jsr);
    pub(crate) const NOP: OpInfo = implied!(Nop, nop);
    pub(crate) const PHA: OpInfo = implied!(Pha, pha);
    pub(crate) const PHP: OpInfo = implied!(Php, php);
    pub(crate) const PLA: OpInfo = implied!(Pla, pla);
    pub(crate) const PLP: OpInfo = implied!(Plp, plp);
    pub(crate) const RTS: OpInfo = implied!(Rts, rts);
}
