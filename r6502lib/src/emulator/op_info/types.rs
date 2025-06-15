use crate::emulator::op_info::op_infos::{Item, CONSTS};
use crate::emulator::{AddressingMode, Cpu, InstructionInfo, Op, OpCycles, Opcode};
use crate::symbols::SymbolInfo;
use anyhow::Result;

#[derive(Clone)]
pub struct OpInfo {
    opcode: Opcode,
    addressing_mode: AddressingMode,
    op: Op,
}

impl OpInfo {
    pub fn iter() -> impl Iterator<Item = &'static OpInfo> {
        CONSTS.iter().map(|(_, item)| match item {
            Item::OpInfo(op) => op,
        })
    }

    #[must_use]
    pub const fn new(opcode: Opcode, addressing_mode: AddressingMode, op: Op) -> Self {
        Self {
            opcode,
            addressing_mode,
            op,
        }
    }

    pub fn execute_no_operand(&self, cpu: &mut Cpu) -> OpCycles {
        self.op.execute_no_operand(cpu)
    }

    pub fn execute_word(&self, cpu: &mut Cpu, value: u16) -> OpCycles {
        self.op.execute_word(cpu, value)
    }

    #[must_use]
    pub const fn opcode(&self) -> Opcode {
        self.opcode
    }

    #[must_use]
    pub const fn op(&self) -> &Op {
        &self.op
    }

    pub fn format_instruction_info(
        &self,
        instruction_info: &InstructionInfo,
        symbols: &[SymbolInfo],
    ) -> Result<String> {
        self.addressing_mode
            .format_instruction_info(instruction_info, symbols)
    }
}
