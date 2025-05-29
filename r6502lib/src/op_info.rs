use crate::{AddressingMode, Op, Opcode};
use constants::{Item, CONSTS};

#[derive(Clone)]
pub(crate) struct OpInfo {
    #[allow(unused)]
    pub(crate) opcode: Opcode,
    #[allow(unused)]
    pub(crate) addressing_mode: AddressingMode,
    #[allow(unused)]
    pub(crate) op: Op,
}

impl OpInfo {
    pub(crate) fn iter() -> impl Iterator<Item = &'static OpInfo> {
        CONSTS.iter().map(|(_, item)| match item {
            Item::OpInfo(op) => op,
        })
    }
}

#[iter_mod::make_items]
mod constants {
    use crate::OpInfo;

    macro_rules! absolute {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Absolute,
                op: $crate::Op::Word($crate::WordOp::Wrapped {
                    wrapper: $crate::absolute,
                    f: $crate::$f,
                }),
            }
        };
    }

    macro_rules! absolute_simple {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Absolute,
                op: $crate::Op::Word($crate::WordOp::Simple { f: $crate::$f }),
            }
        };
    }

    macro_rules! immediate {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Immediate,
                op: $crate::Op::Byte($crate::ByteOp::Simple { f: $crate::$f }),
            }
        };
    }

    macro_rules! implied {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::Implied,
                op: $crate::Op::NoOperand($crate::NoOperandOp { f: $crate::$f }),
            }
        };
    }

    macro_rules! zero_page {
        ($opcode: ident, $f: ident) => {
            $crate::OpInfo {
                opcode: $crate::Opcode::$opcode,
                addressing_mode: $crate::AddressingMode::ZeroPage,
                op: $crate::Op::Byte($crate::ByteOp::Wrapped {
                    wrapper: $crate::zero_page,
                    f: $crate::$f,
                }),
            }
        };
    }

    pub(crate) const ADC_ABS: OpInfo = absolute!(AdcAbs, adc);
    pub(crate) const ADC_IMM: OpInfo = immediate!(AdcImm, adc);
    pub(crate) const ADC_ZP: OpInfo = zero_page!(AdcZp, adc);
    pub(crate) const JMP_ABS: OpInfo = absolute_simple!(JmpAbs, jmp);
    pub(crate) const NOP: OpInfo = implied!(Nop, nop);
}
