use crate::{split_word, AddressingMode, ByteFn, NoOperandFn, Op, SymbolInfo, WordFn};

#[derive(Clone)]
pub(crate) enum Instruction {
    NoOperand(Op, NoOperandFn),
    Byte(Op, ByteFn, u8),
    Word(Op, WordFn, u16),
}

impl Instruction {
    pub(crate) fn pretty_current(&self, symbols: &Vec<SymbolInfo>) -> String {
        match self {
            Self::NoOperand(op, _) => format!(
                "{:02X}       {} ({:?})",
                op.opcode, op.mnemonic, op.addressing_mode
            ),
            Self::Byte(op, _, operand) => {
                fn look_up_name(
                    symbols: &Vec<SymbolInfo>,
                    value: u8,
                    addressing_mode: AddressingMode,
                ) -> String {
                    let temp = value as u16;
                    for symbol in symbols {
                        if symbol.value == temp {
                            return symbol.name.clone();
                        }
                    }
                    addressing_mode.pretty_byte(value)
                }

                format!(
                    "{:02X} {:02X}    {} {} ({:?})",
                    op.opcode,
                    operand,
                    op.mnemonic,
                    look_up_name(symbols, *operand, op.addressing_mode),
                    op.addressing_mode
                )
            }
            Self::Word(op, _, operand) => {
                fn look_up_name(
                    symbols: &Vec<SymbolInfo>,
                    value: u16,
                    addressing_mode: AddressingMode,
                ) -> String {
                    for symbol in symbols {
                        if symbol.value == value {
                            return symbol.name.clone();
                        }
                    }
                    addressing_mode.pretty_word(value)
                }

                let (hi, lo) = split_word(*operand);
                format!(
                    "{:02X} {:02X} {:02X} {} {} ({:?})",
                    op.opcode,
                    lo,
                    hi,
                    op.mnemonic,
                    look_up_name(symbols, *operand, op.addressing_mode),
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
