use crate::{Memory, Reg};

pub(crate) struct VmState {
    pub(crate) reg: Reg,
    pub(crate) memory: Memory,
}
