use crate::emulator::{Binding, Cpu, Instruction, MOS_6502};
use anyhow::{Result, anyhow};
use r6502core::util::split_word;
use r6502cpu::symbols::MapFile;
use r6502cpu::{Opcode, Operand};

#[derive(Clone, Debug)]
pub struct InstructionInfo {
    pub pc: u16,
    pub opcode: Opcode,
    pub operand: Operand,
}

impl InstructionInfo {
    #[must_use]
    pub fn fetch(cpu: &Cpu) -> Self {
        let instruction = Instruction::fetch(cpu);
        InstructionInfo::from_instruction(&instruction)
    }

    #[must_use]
    pub const fn from_instruction(instruction: &Instruction) -> Self {
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

    pub fn display(&self, map_file: &MapFile) -> Result<String> {
        let op_info = MOS_6502
            .get_op_info(&self.opcode)
            .ok_or_else(|| anyhow!("unknown opcode {}", self.opcode))?;
        op_info.format_instruction_info(self, map_file)
    }

    pub fn disassembly(&self, map_file: &MapFile) -> Result<String> {
        let op_info = MOS_6502
            .get_op_info(&self.opcode)
            .ok_or_else(|| anyhow!("unknown opcode {}", self.opcode))?;
        let s = op_info.format_instruction_info(self, map_file)?;
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
    use crate::emulator::InstructionInfo;
    use anyhow::Result;
    use r6502cpu::Opcode::*;
    use r6502cpu::Operand;
    use r6502cpu::symbols::MapFile;

    #[test]
    fn basics() -> Result<()> {
        let map_file = MapFile::default();
        let instruction_info = InstructionInfo {
            pc: 0x1234,
            opcode: Nop,
            operand: Operand::None,
        };
        assert_eq!("NOP", instruction_info.display(&map_file)?);
        assert_eq!(
            "1234  EA        NOP",
            instruction_info.disassembly(&map_file)?
        );

        let instruction_info = InstructionInfo {
            pc: 0x1234,
            opcode: AdcImm,
            operand: Operand::Byte(0x12),
        };
        assert_eq!("ADC #$12", instruction_info.display(&map_file)?);
        assert_eq!(
            "1234  69 12     ADC #$12",
            instruction_info.disassembly(&map_file)?
        );

        let instruction_info = InstructionInfo {
            pc: 0x1234,
            opcode: AdcAbs,
            operand: Operand::Word(0x1234),
        };
        assert_eq!("ADC $1234", instruction_info.display(&map_file)?);
        assert_eq!(
            "1234  6D 34 12  ADC $1234",
            instruction_info.disassembly(&map_file)?
        );

        Ok(())
    }
}
