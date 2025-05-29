use crate::{split_word, Binding, Cpu, Instruction, Opcode, Operand};
use anyhow::{anyhow, Result};

#[allow(unused)]
pub(crate) struct InstructionInfo {
    pub(crate) pc: u16,
    pub(crate) opcode: Opcode,
    pub(crate) operand: Operand,
}

impl InstructionInfo {
    pub(crate) fn from_instruction(instruction: &Instruction) -> Self {
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

    #[allow(unused)]
    pub(crate) fn display(&self, cpu: &Cpu) -> Result<String> {
        let op_info = cpu
            .get_op_info(&self.opcode)
            .ok_or_else(|| anyhow!("Unknown opcode {}", self.opcode))?;
        op_info.addressing_mode.format_instruction_info(self)
    }

    #[allow(unused)]
    pub(crate) fn disassembly(&self, cpu: &Cpu) -> Result<String> {
        let op_info = cpu
            .get_op_info(&self.opcode)
            .ok_or_else(|| anyhow!("Unknown opcode {}", self.opcode))?;
        let s = op_info.addressing_mode.format_instruction_info(self)?;
        Ok(match &self.operand {
            Operand::None => format!("{:04X}  {:2X}        {s}", self.pc, self.opcode as u8),
            Operand::Byte(value) => format!(
                "{:04X}  {:2X} {value:2X}     {s}",
                self.pc, self.opcode as u8,
            ),
            Operand::Word(value) => {
                let (hi, lo) = split_word(*value);
                format!(
                    "{:04X}  {:2X} {lo:2X} {hi:2X}  {s}",
                    self.pc, self.opcode as u8
                )
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Opcode::*;
    use crate::{Cpu, InstructionInfo, Operand};
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let cpu = Cpu::make_6502();

        let instruction_info = InstructionInfo {
            pc: 0x1234,
            opcode: Nop,
            operand: Operand::None,
        };
        assert_eq!("NOP", instruction_info.display(&cpu)?);
        assert_eq!("1234  EA        NOP", instruction_info.disassembly(&cpu)?);

        let instruction_info = InstructionInfo {
            pc: 0x1234,
            opcode: AdcImm,
            operand: Operand::Byte(0x12),
        };
        assert_eq!("ADC #$12", instruction_info.display(&cpu)?);
        assert_eq!(
            "1234  69 12     ADC #$12",
            instruction_info.disassembly(&cpu)?
        );

        let instruction_info = InstructionInfo {
            pc: 0x1234,
            opcode: AdcAbs,
            operand: Operand::Word(0x1234),
        };
        assert_eq!("ADC $1234", instruction_info.display(&cpu)?);
        assert_eq!(
            "1234  6D 34 12  ADC $1234",
            instruction_info.disassembly(&cpu)?
        );

        Ok(())
    }
}
