use crate::{Cpu, Instruction, Opcode};
use anyhow::{anyhow, Result};

#[allow(unused)]
pub(crate) enum InstructionInfo {
    NoOperand { opcode: Opcode },
    Byte { opcode: Opcode, operand: u8 },
    Word { opcode: Opcode, operand: u16 },
}

impl InstructionInfo {
    pub(crate) fn from_instruction(instruction: &Instruction) -> Self {
        match instruction {
            Instruction::NoOperand { f: _, opcode } => Self::NoOperand {
                opcode: opcode.clone(),
            },
            Instruction::Byte {
                f: _,
                opcode,
                operand,
            } => Self::Byte {
                opcode: opcode.clone(),
                operand: operand.clone(),
            },
            Instruction::Word {
                f: _,
                opcode,
                operand,
            } => Self::Word {
                opcode: opcode.clone(),
                operand: operand.clone(),
            },
        }
    }

    // TBD: Format string correctly according to addressing mode
    #[allow(unused)]
    pub(crate) fn display(&self, cpu: &Cpu) -> Result<String> {
        match self {
            Self::NoOperand { opcode } => {
                let op = cpu
                    .get_op(opcode)
                    .ok_or_else(|| anyhow!("Unknown opcode {opcode}"))?;
                Ok(opcode.mnemonic().to_string())
            }
            Self::Byte { opcode, operand } => {
                let op = cpu
                    .get_op(opcode)
                    .ok_or_else(|| anyhow!("Unknown opcode {opcode}"))?;
                Ok(format!("{} BYTE ${operand:02X}", opcode.mnemonic()))
            }
            Self::Word { opcode, operand } => {
                let op = cpu
                    .get_op(opcode)
                    .ok_or_else(|| anyhow!("Unknown opcode {opcode}"))?;
                Ok(format!("{} WORD ${operand:04X}", opcode.mnemonic()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Opcode::*;
    use crate::{Cpu, InstructionInfo};
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let cpu = Cpu::make_6502();

        let instruction_info = InstructionInfo::NoOperand { opcode: Nop };
        assert_eq!("NOP", instruction_info.display(&cpu)?);

        let instruction_info = InstructionInfo::Byte {
            opcode: AdcImm,
            operand: 0x12,
        };
        assert_eq!("ADC BYTE $12", instruction_info.display(&cpu)?);

        let instruction_info = InstructionInfo::Word {
            opcode: AdcAbs,
            operand: 0x1234,
        };
        assert_eq!("ADC WORD $1234", instruction_info.display(&cpu)?);

        Ok(())
    }
}
