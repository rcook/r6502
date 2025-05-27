mod addressing_mode;
mod args;
mod constants;
mod debug_message;
mod flag;
mod image_info;
mod image_start_info;
mod instruction;
mod machine_state;
mod op;
mod op_func;
mod ops;
mod register_file;
mod run;
mod status;
mod status_message;
mod types;
mod ui;
mod util;
mod vm;

pub(crate) use addressing_mode::AddressingMode;
pub(crate) use args::Args;
pub(crate) use constants::{IRQ, OSHALT, OSWRCH, STACK_BASE};
pub(crate) use debug_message::DebugMessage;
pub(crate) use flag::Flag;
pub(crate) use image_info::ImageInfo;
pub(crate) use image_start_info::ImageStartInfo;
pub(crate) use instruction::Instruction;
pub(crate) use machine_state::MachineState;
pub(crate) use op::Op;
pub(crate) use op_func::{ByteFn, Cycles, NoOperandFn, OpFunc, WordFn};
pub(crate) use ops::iter_ops;
pub(crate) use register_file::RegisterFile;
pub(crate) use status::Status;
pub(crate) use status_message::StatusMessage;
pub(crate) use types::Memory;
pub(crate) use ui::UI;
pub(crate) use util::{make_word, split_word};
pub(crate) use vm::run_vm;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
