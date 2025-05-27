use crate::MachineState;

pub(crate) type Cycles = u32;
pub(crate) type NoOperandFn = fn(&mut MachineState) -> Cycles;
pub(crate) type ByteFn = fn(&mut MachineState, u8) -> Cycles;
pub(crate) type WordFn = fn(&mut MachineState, u16) -> Cycles;

#[derive(Clone, Copy)]
pub(crate) enum OpFunc {
    NoOperand(NoOperandFn),
    Byte(ByteFn),
    Word(WordFn),
}
