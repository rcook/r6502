use crate::{split_word, ByteFn, NoOperandFn, Op, WordFn};

#[derive(Clone)]
pub(crate) enum Instruction {
    NoOperand(Op, NoOperandFn),
    Byte(Op, ByteFn, u8),
    Word(Op, WordFn, u16),
}

impl Instruction {
    pub(crate) fn pretty_current(&self) -> String {
        match self {
            Self::NoOperand(op, _) => format!(
                "{:02X}       {} ({:?})",
                op.opcode, op.mnemonic, op.addressing_mode
            ),
            Self::Byte(op, _, operand) => format!(
                "{:02X} {:02X}    {} {} ({:?})",
                op.opcode,
                operand,
                op.mnemonic,
                op.addressing_mode.pretty_byte(*operand),
                op.addressing_mode
            ),
            Self::Word(op, _, operand) => {
                let (hi, lo) = split_word(*operand);
                format!(
                    "{:02X} {:02X} {:02X} {} {} ({:?})",
                    op.opcode,
                    lo,
                    hi,
                    op.mnemonic,
                    op.addressing_mode.pretty_word(*operand),
                    op.addressing_mode
                )
            }
        }
    }

    pub(crate) fn pretty_disassembly(&self) -> String {
        match self {
            Self::NoOperand(op, _) => {
                format!("{:02X}       {}", op.opcode, op.mnemonic)
            }
            Self::Byte(op, _, operand) => {
                format!(
                    "{:02X} {:02X}    {} {}",
                    op.opcode,
                    operand,
                    op.mnemonic,
                    op.addressing_mode.pretty_byte(*operand)
                )
            }
            Self::Word(op, _, operand) => {
                let (hi, lo) = split_word(*operand);
                format!(
                    "{:02X} {:02X} {:02X} {} {}",
                    op.opcode,
                    lo,
                    hi,
                    op.mnemonic,
                    op.addressing_mode.pretty_word(*operand)
                )
            }
        }
    }
}
