use crate::Cpu;

pub(crate) type NoOperandFn = fn(&mut Cpu) -> ();
pub(crate) type ByteFn = fn(&mut Cpu, u8) -> ();
pub(crate) type WordFn = fn(&mut Cpu, u16) -> ();

#[derive(Clone, Copy)]
pub(crate) enum OpFunc {
    NoOperand(NoOperandFn),
    Byte(ByteFn),
    Word(WordFn),
}
