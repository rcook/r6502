use crate::{Op, OpInfo, Opcode};
use std::collections::HashMap;

pub(crate) struct Cpu(HashMap<Opcode, Op>);

impl Cpu {
    // http://www.6502.org/users/obelisk/6502/instructions.html
    #[allow(unused)]
    pub(crate) fn make_6502() -> Self {
        use crate::Opcode::*;

        Self(
            OpInfo::iter()
                .map(|o| (o.opcode.clone(), o.op.clone()))
                .collect::<HashMap<_, _>>(),
        )
    }

    pub(crate) fn get_op(&self, opcode: &Opcode) -> Option<&Op> {
        self.0.get(opcode)
    }
}
