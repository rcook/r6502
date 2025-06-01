mod args;
mod debug_message;
mod io_message;
mod monitor_message;
mod poll_result;
mod run;
mod status;
mod symbol_info;
mod ui;
mod ui_host;
mod ui_monitor;
mod util;
mod vm_status;

pub(crate) use args::Args;
pub(crate) use debug_message::DebugMessage;
pub(crate) use io_message::IoMessage;
pub(crate) use monitor_message::MonitorMessage;
pub(crate) use poll_result::PollResult;
pub(crate) use status::Status;
pub(crate) use symbol_info::SymbolInfo;
pub(crate) use ui::Ui;
pub(crate) use ui_host::UiHost;
pub(crate) use ui_monitor::UiMonitor;
pub(crate) use util::initialize_vm;
pub(crate) use vm_status::VmStatus;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
