use crate::{OpInfo, Opcode};
use std::{collections::HashMap, sync::LazyLock};

pub static MOS_6502: LazyLock<InstructionSet> = LazyLock::new(|| {
    InstructionSet(
        OpInfo::iter()
            .map(|o| (o.opcode(), o.clone()))
            .collect::<HashMap<_, _>>(),
    )
});

pub struct InstructionSet(HashMap<Opcode, OpInfo>);

impl InstructionSet {
    pub fn get_op_info(&self, opcode: &Opcode) -> Option<&OpInfo> {
        self.0.get(opcode)
    }
}
