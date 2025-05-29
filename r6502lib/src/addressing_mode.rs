use crate::InstructionInfo;
use anyhow::{bail, Result};

#[derive(Clone)]
pub(crate) enum AddressingMode {
    Absolute,
    Immediate,
    Implied,
    ZeroPage,
}

impl AddressingMode {
    pub(crate) fn format_instruction_info(
        &self,
        instruction_info: &InstructionInfo,
    ) -> Result<String> {
        match self {
            Self::Absolute => match instruction_info {
                InstructionInfo::Word { opcode, operand } => {
                    Ok(format!("{} ${:04X}", opcode.mnemonic(), operand))
                }
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode()),
            },
            Self::Immediate => match instruction_info {
                InstructionInfo::Byte { opcode, operand } => {
                    Ok(format!("{} #${:02X}", opcode.mnemonic(), operand))
                }
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode()),
            },
            Self::Implied => match instruction_info {
                InstructionInfo::NoOperand { opcode } => Ok(String::from(opcode.mnemonic())),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode()),
            },
            Self::ZeroPage => match instruction_info {
                InstructionInfo::Byte { opcode, operand } => {
                    Ok(format!("{} ${:02X}", opcode.mnemonic(), operand))
                }
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode()),
            },
        }
    }
}
