use crate::op_info::op_infos::{Item, CONSTS};
use crate::{AddressingMode, InstructionInfo, Op, OpCycles, Opcode, SymbolInfo, VmState};
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

    pub(crate) fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub(crate) fn op(&self) -> &Op {
        &self.op
    }

    pub(crate) fn execute_no_operand(&self, s: &mut VmState) -> OpCycles {
        self.op.execute_no_operand(s)
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
