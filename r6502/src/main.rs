#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::pedantic)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::verbose_bit_mask)]
#![allow(missing_docs)]

mod args;
mod command;
mod debug_message;
mod gui;
mod io_message;
mod monitor_message;
mod run;
mod state;
mod terminal;
mod ui;
mod ui_host;
mod ui_monitor;

pub(crate) use args::Args;
pub(crate) use command::Command;
pub(crate) use debug_message::DebugMessage;
pub(crate) use gui::run_gui;
pub(crate) use io_message::IoMessage;
pub(crate) use monitor_message::MonitorMessage;
pub(crate) use state::State;
pub(crate) use terminal::run_terminal;
pub(crate) use ui::Ui;
pub(crate) use ui_host::UiHost;
pub(crate) use ui_monitor::UiMonitor;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
