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
    use crate::AddressingMode::*;
    use crate::Op::*;
    use crate::Opcode::*;
    use crate::{absolute, adc, jmp, nop, zero_page, ByteOp, NoOperandOp, OpInfo, WordOp};

    pub(crate) const ADC_ABS: OpInfo = OpInfo {
        opcode: AdcAbs,
        addressing_mode: Absolute,
        op: Word(WordOp::Wrapped {
            wrapper: absolute,
            f: adc,
        }),
    };

    pub(crate) const ADC_IMM: OpInfo = OpInfo {
        opcode: AdcImm,
        addressing_mode: Immediate,
        op: Byte(ByteOp::Simple { f: adc }),
    };

    pub(crate) const ADC_ZP: OpInfo = OpInfo {
        opcode: AdcZp,
        addressing_mode: ZeroPage,
        op: Byte(ByteOp::Wrapped {
            wrapper: zero_page,
            f: adc,
        }),
    };

    pub(crate) const JMP_ABS: OpInfo = OpInfo {
        opcode: JmpAbs,
        addressing_mode: Absolute,
        op: Word(WordOp::Simple { f: jmp }),
    };

    pub(crate) const NOP: OpInfo = OpInfo {
        opcode: Nop,
        addressing_mode: Implied,
        op: NoOperand(NoOperandOp { f: nop }),
    };
}
