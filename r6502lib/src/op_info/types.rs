use crate::op_info::op_infos::{Item, CONSTS};
use crate::{AddressingMode, Op, Opcode};

#[derive(Clone)]
pub struct OpInfo {
    pub(crate) opcode: Opcode,
    pub(crate) addressing_mode: AddressingMode,
    pub op: Op,
}

impl OpInfo {
    pub(crate) fn iter() -> impl Iterator<Item = &'static OpInfo> {
        CONSTS.iter().map(|(_, item)| match item {
            Item::OpInfo(op) => op,
        })
    }
}
