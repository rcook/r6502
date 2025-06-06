mod address_range;
mod args;
mod command;
mod debug_message;
mod gui;
mod io_message;
mod monitor_message;
mod pia;
mod run;
mod state;
mod terminal;
mod ui;
mod ui_host;
mod ui_monitor;

pub(crate) use address_range::AddressRange;
pub(crate) use args::Args;
pub(crate) use command::Command;
pub(crate) use debug_message::DebugMessage;
pub(crate) use gui::run_gui;
pub(crate) use io_message::IoMessage;
pub(crate) use monitor_message::MonitorMessage;
pub(crate) use pia::run_pia;
pub(crate) use state::State;
pub(crate) use terminal::run_terminal;
pub(crate) use ui::Ui;
pub(crate) use ui_host::UiHost;
pub(crate) use ui_monitor::UiMonitor;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
