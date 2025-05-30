use crate::{OpInfo, Opcode};
use std::collections::HashMap;

pub(crate) struct Cpu(HashMap<Opcode, OpInfo>);

impl Cpu {
    // http://www.6502.org/users/obelisk/6502/instructions.html
    #[allow(unused)]
    pub(crate) fn make_6502() -> Self {
        use crate::Opcode::*;

        Self(
            OpInfo::iter()
                .map(|o| (o.opcode, o.clone()))
                .collect::<HashMap<_, _>>(),
        )
    }

    pub(crate) fn get_op_info(&self, opcode: &Opcode) -> Option<&OpInfo> {
        self.0.get(opcode)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::make_6502()
    }
}
