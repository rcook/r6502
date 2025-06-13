use crate::machine_config::MachineInfo;
use crate::tui::ui::Ui;
use crate::tui::ui_host::UiHost;
use anyhow::Result;
use r6502lib::{Image, SymbolInfo};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::spawn;

pub(crate) fn run_gui(path: &Path, load: Option<u16>, start: Option<u16>) -> Result<()> {
    let image = Image::load(path, load, start, None)?;
    let symbols = SymbolInfo::load(path)?;
    let debug_channel = channel();
    let monitor_channel = channel();
    let io_channel = channel();
    let mut ui = Ui::new(monitor_channel.1, io_channel.1, &debug_channel.0, symbols);
    spawn(move || {
        let machine_info = MachineInfo::load(&Some(String::from("Acorn"))).expect("Must succeed");
        let (bus, _) = machine_info.create_bus(&image).expect("Must succeed");
        UiHost::new(bus, debug_channel.1, monitor_channel.0, io_channel.0)
            .run(image.start)
            .expect("Must succeed");
    });
    ui.run();
    Ok(())
}
