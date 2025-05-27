use crate::Cpu;

#[derive(Clone, Copy)]
pub(crate) enum OpFunc {
    NoArgs(fn(&mut Cpu) -> ()),
    Byte(fn(&mut Cpu, u8) -> ()),
    Word(fn(&mut Cpu, u16) -> ()),
}
