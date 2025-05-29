use crate::{InstructionInfo, Operand};
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
            Self::Absolute => match instruction_info.operand {
                Operand::Word(value) => Ok(format!(
                    "{} ${:04X}",
                    instruction_info.opcode.mnemonic(),
                    value
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::Immediate => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} #${:02X}",
                    instruction_info.opcode.mnemonic(),
                    value
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::Implied => match instruction_info.operand {
                Operand::None => Ok(String::from(instruction_info.opcode.mnemonic())),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
            Self::ZeroPage => match instruction_info.operand {
                Operand::Byte(value) => Ok(format!(
                    "{} ${:02X}",
                    instruction_info.opcode.mnemonic(),
                    value
                )),
                _ => bail!("invalid addressing mode for {}", instruction_info.opcode),
            },
        }
    }
}
