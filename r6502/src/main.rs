#![allow(unused)]

mod addressing_mode;
mod args;
mod debug_message;
mod flag;
mod image_info;
mod image_source;
mod instruction;
mod io_message;
mod machine_state;
mod monitor_message;
mod op;
mod op_func;
mod ops;
mod register_file;
mod run;
mod status;
mod symbol_info;
mod test_host;
mod types;
mod ui;
mod ui_host;
mod util;
mod vm_host;
mod vm_status;

pub(crate) use addressing_mode::AddressingMode;
pub(crate) use args::Args;
pub(crate) use debug_message::DebugMessage;
pub(crate) use flag::Flag;
pub(crate) use image_info::ImageInfo;
pub(crate) use image_source::ImageSource;
pub(crate) use instruction::Instruction;
pub(crate) use io_message::IoMessage;
pub(crate) use machine_state::MachineState;
pub(crate) use monitor_message::MonitorMessage;
pub(crate) use op::Op;
pub(crate) use op_func::{ByteFn, Cycles, NoOperandFn, OpFunc, WordFn};
pub(crate) use ops::iter_ops;
pub(crate) use register_file::RegisterFile;
pub(crate) use status::Status;
pub(crate) use symbol_info::SymbolInfo;
pub(crate) use test_host::TestHost;
pub(crate) use types::Memory;
pub(crate) use ui::Ui;
pub(crate) use ui_host::UiHost;
pub(crate) use util::{compute_branch, make_word, split_word};
pub(crate) use vm_host::{PollResult, VmHost};
pub(crate) use vm_status::VmStatus;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
