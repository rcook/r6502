use crate::emulator::run_emulator;
use crate::font::Font;
use crate::tui::TextTerminal;
use crate::util::set_main_thread;
use anyhow::{Result, anyhow};
use log::info;
use path_absolutize::Absolutize;
use r6502config::HostHookType;
use r6502core::emulator::{Bus, Cpu};
use r6502core::{BusDevice, DeviceMapping};
use r6502hw::MachineInfo;
use r6502lib::util::make_word;
use r6502lib::{AddressRange, Channel, RESET};
use r6502snapshot::MemoryImage;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::path::Path;

pub fn run_tui() -> Result<()> {
    set_main_thread();
    info!("main thread started");
    TextTerminal::with(run_emulator)
}
