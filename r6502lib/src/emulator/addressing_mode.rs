use crate::emulator::{InstructionInfo, Operand, SymbolInfo};
use anyhow::{bail, Result};

#[derive(Clone)]
pub(crate) enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Implied,
    IndexedIndirectX,
    Indirect,
    IndirectIndexedY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

impl AddressingMode {
    pub(crate) fn format_instruction_info(
        &self,
        instruction_info: &InstructionInfo,
        symbols: &[SymbolInfo],
    ) -> Result<String> {
        match self {
            Self::Absolute => match instruction_info.operand {
                Operand::Word(value) => Ok(format!(
                    "{} {}",
                    instruction_info.opcode.mnemonic(),
                    Self::format_addr(symbols, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::AbsoluteX => match instruction_info.operand {
                Operand::Word(value) => Ok(format!(
                    "{} {},X",
                    instruction_info.opcode.mnemonic(),
                    Self::format_addr(symbols, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::AbsoluteY => match instruction_info.operand {
                Operand::Word(value) => Ok(format!(
                    "{} {},Y",
                    instruction_info.opcode.mnemonic(),
                    Self::format_addr(symbols, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::Accumulator => match instruction_info.operand {
                Operand::None => Ok(format!("{} A", instruction_info.opcode.mnemonic())),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::Immediate => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} #{}",
                    instruction_info.opcode.mnemonic(),
                    Self::format_byte(value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::Implied => match instruction_info.operand {
                Operand::None => Ok(String::from(instruction_info.opcode.mnemonic())),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::IndexedIndirectX => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} ({},X)",
                    instruction_info.opcode.mnemonic(),
                    Self::format_zero_page_addr(symbols, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::Indirect => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} ({})",
                    instruction_info.opcode.mnemonic(),
                    Self::format_zero_page_addr(symbols, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::IndirectIndexedY => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} ({}),Y",
                    instruction_info.opcode.mnemonic(),
                    Self::format_zero_page_addr(symbols, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::Relative => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} {}",
                    instruction_info.opcode.mnemonic(),
                    Self::format_branch(symbols, value, instruction_info.pc)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::ZeroPage => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} {}",
                    instruction_info.opcode.mnemonic(),
                    Self::format_zero_page_addr(symbols, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::ZeroPageX => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} {},X",
                    instruction_info.opcode.mnemonic(),
                    Self::format_zero_page_addr(symbols, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::ZeroPageY => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} {},Y",
                    instruction_info.opcode.mnemonic(),
                    Self::format_zero_page_addr(symbols, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
        }
    }

    const fn compute_branch(pc: u16, operand: u8) -> u16 {
        let lhs = pc as i32;
        let rhs = (operand as i8) as i32;
        (lhs + rhs) as u16
    }

    fn find_name(symbols: &[SymbolInfo], value: u16) -> Option<String> {
        for symbol in symbols {
            if symbol.value == value {
                return Some(symbol.name.clone());
            }
        }
        None
    }

    fn format_byte(value: u8) -> String {
        format!("${value:02X}")
    }

    fn format_addr(symbols: &[SymbolInfo], value: u16) -> String {
        Self::find_name(symbols, value).unwrap_or_else(|| format!("${value:04X}"))
    }

    fn format_branch(symbols: &[SymbolInfo], value: u8, pc: u16) -> String {
        let effective_value = Self::compute_branch(pc + 2, value);
        Self::find_name(symbols, effective_value)
            .unwrap_or_else(|| format!("${effective_value:04X}"))
    }

    fn format_zero_page_addr(symbols: &[SymbolInfo], value: u8) -> String {
        Self::find_name(symbols, value as u16).unwrap_or_else(|| format!("${value:02X}"))
    }
}
