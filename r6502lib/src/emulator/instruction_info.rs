use crate::emulator::util::split_word;
use crate::emulator::{Binding, Cpu, Instruction, Opcode, Operand, SymbolInfo, MOS_6502};
use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct InstructionInfo {
    pub(crate) pc: u16,
    pub(crate) opcode: Opcode,
    pub(crate) operand: Operand,
}

impl InstructionInfo {
    #[must_use]
    pub fn fetch(cpu: &Cpu) -> Self {
        let instruction = Instruction::fetch(cpu);
        InstructionInfo::from_instruction(&instruction)
    }

    pub(crate) const fn from_instruction(instruction: &Instruction) -> Self {
        match &instruction.binding {
            Binding::NoOperand(_) => Self {
                pc: instruction.pc,
                opcode: instruction.opcode,
                operand: Operand::None,
            },
            Binding::Byte(_, value) => Self {
                pc: instruction.pc,
                opcode: instruction.opcode,
                operand: Operand::Byte(*value),
            },
            Binding::Word(_, value) => Self {
                pc: instruction.pc,
                opcode: instruction.opcode,
                operand: Operand::Word(*value),
            },
        }
    }

    pub fn display(&self, symbols: &[SymbolInfo]) -> Result<String> {
        let op_info = MOS_6502
            .get_op_info(&self.opcode)
            .ok_or_else(|| anyhow!("Unknown opcode {}", self.opcode))?;
        op_info.format_instruction_info(self, symbols)
    }

    pub fn disassembly(&self, symbols: &[SymbolInfo]) -> Result<String> {
        let op_info = MOS_6502
            .get_op_info(&self.opcode)
            .ok_or_else(|| anyhow!("Unknown opcode {}", self.opcode))?;
        let s = op_info.format_instruction_info(self, symbols)?;
        Ok(match &self.operand {
            Operand::None => format!("{:04X}  {:02X}        {s}", self.pc, self.opcode as u8),
            Operand::Byte(value) => format!(
                "{:04X}  {:02X} {value:02X}     {s}",
                self.pc, self.opcode as u8,
            ),
            Operand::Word(value) => {
                let (hi, lo) = split_word(*value);
                format!(
                    "{:04X}  {:02X} {lo:02X} {hi:02X}  {s}",
                    self.pc, self.opcode as u8
                )
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::Opcode::*;
    use crate::emulator::{InstructionInfo, Operand};
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let symbols = Vec::new();
        let instruction_info = InstructionInfo {
            pc: 0x1234,
            opcode: Nop,
            operand: Operand::None,
        };
        assert_eq!("NOP", instruction_info.display(&symbols)?);
        assert_eq!(
            "1234  EA        NOP",
            instruction_info.disassembly(&symbols)?
        );

        let instruction_info = InstructionInfo {
            pc: 0x1234,
            opcode: AdcImm,
            operand: Operand::Byte(0x12),
        };
        assert_eq!("ADC #$12", instruction_info.display(&symbols)?);
        assert_eq!(
            "1234  69 12     ADC #$12",
            instruction_info.disassembly(&symbols)?
        );

        let instruction_info = InstructionInfo {
            pc: 0x1234,
            opcode: AdcAbs,
            operand: Operand::Word(0x1234),
        };
        assert_eq!("ADC $1234", instruction_info.display(&symbols)?);
        assert_eq!(
            "1234  6D 34 12  ADC $1234",
            instruction_info.disassembly(&symbols)?
        );

        Ok(())
    }
}
