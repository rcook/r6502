use crate::{AddressingMode, Op, Opcode};
use inner::{Item, CONSTS};

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
mod inner {
    use crate::{absolute, adc, jmp, nop, zero_page, AddressingMode, Op, OpInfo, OpWord};
    use crate::{OpByte, Opcode::*};

    pub(crate) const ADC_ABS: OpInfo = OpInfo {
        opcode: AdcAbs,
        addressing_mode: AddressingMode::Absolute,
        op: Op::Word(OpWord::Wrapped {
            wrapper: absolute,
            f: adc,
        }),
    };

    pub(crate) const ADC_IMM: OpInfo = OpInfo {
        opcode: AdcImm,
        addressing_mode: AddressingMode::Immediate,
        op: Op::Byte(OpByte::Simple { f: adc }),
    };

    pub(crate) const ADC_ZP: OpInfo = OpInfo {
        opcode: AdcZp,
        addressing_mode: AddressingMode::ZeroPage,
        op: Op::Byte(OpByte::Wrapped {
            wrapper: zero_page,
            f: adc,
        }),
    };

    pub(crate) const JMP_ABS: OpInfo = OpInfo {
        opcode: JmpAbs,
        addressing_mode: AddressingMode::Absolute,
        op: Op::Word(OpWord::Simple { f: jmp }),
    };

    pub(crate) const NOP: OpInfo = OpInfo {
        opcode: Nop,
        addressing_mode: AddressingMode::Implied,
        op: Op::NoOperand { f: nop },
    };
}
