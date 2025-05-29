use crate::op_info::op_infos::{Item, CONSTS};
use crate::{AddressingMode, Op, Opcode};

#[derive(Clone)]
pub(crate) struct OpInfo {
    #[allow(unused)]
    pub(crate) opcode: Opcode,
    #[allow(unused)]
    pub(crate) addressing_mode: AddressingMode,
    #[allow(unused)]
    pub(crate) op: Op,
}

impl OpInfo {
    pub(crate) fn iter() -> impl Iterator<Item = &'static OpInfo> {
        CONSTS.iter().map(|(_, item)| match item {
            Item::OpInfo(op) => op,
        })
    }
}
