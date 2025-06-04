mod address_range;
mod args;
mod command;
mod debug_message;
mod io_message;
mod monitor_message;
mod run;
mod state;
mod ui;
mod ui_host;
mod ui_monitor;

pub(crate) use address_range::AddressRange;
pub(crate) use args::Args;
pub(crate) use command::Command;
pub(crate) use debug_message::DebugMessage;
pub(crate) use io_message::IoMessage;
pub(crate) use monitor_message::MonitorMessage;
pub(crate) use state::State;
pub(crate) use ui::Ui;
pub(crate) use ui_host::UiHost;
pub(crate) use ui_monitor::UiMonitor;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
