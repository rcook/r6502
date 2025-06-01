use crate::{OpInfo, Opcode};
use std::{collections::HashMap, sync::LazyLock};

pub static MOS_6502: LazyLock<Cpu> = LazyLock::new(|| {
    Cpu(OpInfo::iter()
        .map(|o| (o.opcode(), o.clone()))
        .collect::<HashMap<_, _>>())
});

pub struct Cpu(HashMap<Opcode, OpInfo>);

impl Cpu {
    pub fn get_op_info(&self, opcode: &Opcode) -> Option<&OpInfo> {
        self.0.get(opcode)
    }
}
