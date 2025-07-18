use crate::emulator::{InstructionInfo, Operand};
use crate::num::{Truncate, Wrap};
use crate::symbols::MapFile;
use anyhow::{Result, bail};

#[derive(Clone)]
pub enum AddressingMode {
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
    pub fn format_instruction_info(
        &self,
        instruction_info: &InstructionInfo,
        map_file: &MapFile,
    ) -> Result<String> {
        match self {
            Self::Absolute => match instruction_info.operand {
                Operand::Word(value) => Ok(format!(
                    "{} {}",
                    instruction_info.opcode.mnemonic(),
                    Self::format_addr(map_file, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::AbsoluteX => match instruction_info.operand {
                Operand::Word(value) => Ok(format!(
                    "{} {},X",
                    instruction_info.opcode.mnemonic(),
                    Self::format_addr(map_file, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::AbsoluteY => match instruction_info.operand {
                Operand::Word(value) => Ok(format!(
                    "{} {},Y",
                    instruction_info.opcode.mnemonic(),
                    Self::format_addr(map_file, value)
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
                    Self::format_zero_page_addr(map_file, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::Indirect => match instruction_info.operand {
                Operand::Word(value) => Ok(format!(
                    "{} ({})",
                    instruction_info.opcode.mnemonic(),
                    Self::format_addr(map_file, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::IndirectIndexedY => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} ({}),Y",
                    instruction_info.opcode.mnemonic(),
                    Self::format_zero_page_addr(map_file, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::Relative => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} {}",
                    instruction_info.opcode.mnemonic(),
                    Self::format_branch(map_file, value, instruction_info.pc)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::ZeroPage => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} {}",
                    instruction_info.opcode.mnemonic(),
                    Self::format_zero_page_addr(map_file, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::ZeroPageX => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} {},X",
                    instruction_info.opcode.mnemonic(),
                    Self::format_zero_page_addr(map_file, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::ZeroPageY => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} {},Y",
                    instruction_info.opcode.mnemonic(),
                    Self::format_zero_page_addr(map_file, value)
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
        }
    }

    fn compute_branch(pc: u16, operand: u8) -> u16 {
        let lhs = i32::from(pc);
        let rhs = i32::from(i8::wrap(operand));
        u16::truncate(lhs + rhs)
    }

    fn find_name(map_file: &MapFile, value: u16) -> Option<String> {
        let temp = u32::from(value);
        for export in &map_file.exports {
            if export.value == temp {
                return Some(export.name.clone());
            }
        }
        None
    }

    fn format_byte(value: u8) -> String {
        format!("${value:02X}")
    }

    fn format_addr(map_file: &MapFile, value: u16) -> String {
        Self::find_name(map_file, value).unwrap_or_else(|| format!("${value:04X}"))
    }

    fn format_branch(map_file: &MapFile, value: u8, pc: u16) -> String {
        let effective_value = Self::compute_branch(pc + 2, value);
        Self::find_name(map_file, effective_value)
            .unwrap_or_else(|| format!("${effective_value:04X}"))
    }

    fn format_zero_page_addr(map_file: &MapFile, value: u8) -> String {
        Self::find_name(map_file, u16::from(value)).unwrap_or_else(|| format!("${value:02X}"))
    }
}
