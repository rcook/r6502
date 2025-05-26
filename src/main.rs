mod addressing_mode;
mod constants;
mod demo;
mod flag;
mod ops;
mod state;
mod types;

pub(crate) use addressing_mode::AddressingMode;
pub(crate) use constants::{IRQ, OSHALT, OSWRCH, STACK_BASE};
pub(crate) use flag::Flag;
pub(crate) use ops::{Op, OPS};
pub(crate) use state::State;
pub(crate) use types::{Memory, OpFn};

fn main() -> anyhow::Result<()> {
    crate::demo::demo()
}
