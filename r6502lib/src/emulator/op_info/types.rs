use crate::emulator::op_info::op_infos::{Item, CONSTS};
use crate::emulator::{AddressingMode, Cpu, InstructionInfo, Op, OpCycles, Opcode, SymbolInfo};
use anyhow::Result;

#[derive(Clone)]
pub struct OpInfo {
    opcode: Opcode,
    addressing_mode: AddressingMode,
    op: Op,
}

impl OpInfo {
    pub(crate) fn iter() -> impl Iterator<Item = &'static OpInfo> {
        CONSTS.iter().map(|(_, item)| match item {
            Item::OpInfo(op) => op,
        })
    }

    pub(crate) const fn new(opcode: Opcode, addressing_mode: AddressingMode, op: Op) -> Self {
        Self {
            opcode,
            addressing_mode,
            op,
        }
    }

    pub fn execute_no_operand(&self, cpu: &mut Cpu) -> OpCycles {
        self.op.execute_no_operand(cpu)
    }

    pub(crate) const fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub(crate) const fn op(&self) -> &Op {
        &self.op
    }

    pub(crate) fn format_instruction_info(
        &self,
        instruction_info: &InstructionInfo,
        symbols: &[SymbolInfo],
    ) -> Result<String> {
        self.addressing_mode
            .format_instruction_info(instruction_info, symbols)
    }
}
