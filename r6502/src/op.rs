use crate::{AddressingMode, OpFunc};

#[derive(Clone, Copy)]
pub(crate) struct Op {
    #[allow(unused)]
    pub(crate) mnemonic: &'static str,
    #[allow(unused)]
    pub(crate) addressing_mode: AddressingMode,
    pub(crate) opcode: u8,
    pub(crate) func: OpFunc,
}
