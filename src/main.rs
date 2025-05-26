mod addressing_mode;
mod constants;
mod controller;
mod controller_message;
mod demo;
mod flag;
mod ops;
mod state;
mod types;
mod ui;
mod ui_message;

pub(crate) use addressing_mode::AddressingMode;
pub(crate) use constants::{IRQ, OSHALT, OSWRCH, STACK_BASE};
pub(crate) use controller::Controller;
pub(crate) use controller_message::ControllerMessage;
pub(crate) use flag::Flag;
pub(crate) use ops::{Op, OPS};
pub(crate) use state::State;
pub(crate) use types::{Memory, OpFn};
pub(crate) use ui::UI;
pub(crate) use ui_message::UIMessage;

fn main() -> anyhow::Result<()> {
    crate::demo::demo()
}
