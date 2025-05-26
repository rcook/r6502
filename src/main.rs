mod addressing_mode;
mod constants;
mod controller;
mod controller_message;
mod cpu;
mod cpu_message;
mod demo;
mod flag;
mod ops;
mod thunk;
mod types;
mod ui;
mod ui_message;
mod vm;

pub(crate) use addressing_mode::AddressingMode;
pub(crate) use constants::{IRQ, OSHALT, OSWRCH, STACK_BASE};
pub(crate) use controller::Controller;
pub(crate) use controller_message::ControllerMessage;
pub(crate) use cpu::Cpu;
pub(crate) use cpu_message::CpuMessage;
pub(crate) use flag::Flag;
pub(crate) use ops::{Op, OPS};
pub(crate) use thunk::Thunk;
pub(crate) use types::{Memory, OpFn};
pub(crate) use ui::UI;
pub(crate) use ui_message::UIMessage;
pub(crate) use vm::run;

fn main() -> anyhow::Result<()> {
    crate::demo::demo()
}
