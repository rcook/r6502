mod constants;
mod demo;
mod flag;
mod ops;
mod state;
mod types;

pub(crate) use constants::{OSHALT, OSWRCH, STACK_BASE};
pub(crate) use flag::Flag;
pub(crate) use state::State;
pub(crate) use types::{Memory, OpFn};

fn main() -> anyhow::Result<()> {
    crate::demo::demo()
}
