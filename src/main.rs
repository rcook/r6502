mod addressing_mode;
mod args;
mod cli_host;
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
mod run_vm_result;
mod status;
mod status_message;
mod test_host;
mod types;
mod ui;
mod ui_host;
mod util;
mod vm;
mod vm_host;

pub(crate) use addressing_mode::AddressingMode;
pub(crate) use args::Args;
pub(crate) use cli_host::CliHost;
pub(crate) use constants::{IRQ, IRQ_VALUE, OSHALT, OSWRCH, STACK_BASE};
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
pub(crate) use run_vm_result::{RunVMResult, RunVMStatus};
pub(crate) use status::Status;
pub(crate) use status_message::StatusMessage;
pub(crate) use test_host::TestHost;
pub(crate) use types::Memory;
pub(crate) use ui::UI;
pub(crate) use ui_host::UIHost;
pub(crate) use util::{make_word, split_word};
pub(crate) use vm::run_vm;
pub(crate) use vm_host::{PollResult, VMHost};

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
