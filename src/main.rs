mod addressing_mode;
mod args;
mod constants;
mod controller;
mod controller_message;
mod cpu;
mod cpu_message;
mod demo;
mod flag;
mod op_func;
mod ops;
mod program_info;
mod types;
mod ui;
mod ui_message;
mod util;
mod vm;

pub(crate) use addressing_mode::AddressingMode;
pub(crate) use args::Args;
pub(crate) use constants::{IRQ, OSHALT, OSWRCH, STACK_BASE};
pub(crate) use controller::Controller;
pub(crate) use controller_message::ControllerMessage;
pub(crate) use cpu::Cpu;
pub(crate) use cpu_message::CpuMessage;
pub(crate) use flag::Flag;
pub(crate) use op_func::OpFunc;
pub(crate) use ops::{Op, OPS};
pub(crate) use program_info::ProgramInfo;
pub(crate) use types::Memory;
pub(crate) use ui::UI;
pub(crate) use ui_message::UIMessage;
pub(crate) use util::{make_word, split_word};
pub(crate) use vm::run_vm;

fn main() -> anyhow::Result<()> {
    crate::demo::demo()
}
