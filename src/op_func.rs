use crate::Cpu;

pub(crate) type Cycles = u32;
pub(crate) type NoOperandFn = fn(&mut Cpu) -> Cycles;
pub(crate) type ByteFn = fn(&mut Cpu, u8) -> Cycles;
pub(crate) type WordFn = fn(&mut Cpu, u16) -> Cycles;

#[derive(Clone, Copy)]
pub(crate) enum OpFunc {
    NoOperand(NoOperandFn),
    Byte(ByteFn),
    Word(WordFn),
}
