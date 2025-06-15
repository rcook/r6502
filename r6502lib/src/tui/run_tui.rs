use crate::debug_options::DebugOptions;
use crate::emulator::{Image, TuiInputQueue};
use crate::machine_config::MachineInfo;
use crate::symbols::SymbolInfo;
use crate::tui::cursive_tui::CursiveTui;
use crate::tui::tui_host::TuiHost;
use anyhow::Result;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

pub fn run_tui(opts: &DebugOptions) -> Result<()> {
    let image = Image::load(&opts.path, opts.load, opts.start, None)?;
    let machine_info = match image.tag {
        Some(tag) => MachineInfo::find_by_tag(tag)?,
        None => MachineInfo::find_by_name(&opts.machine)?,
    };

    // TBD: Use MapFile instead!
    let symbols = SymbolInfo::load(&opts.path)?;
    let debug_channel = channel();
    let monitor_channel = channel();
    let io_channel = channel();
    let mut ui = CursiveTui::new(monitor_channel.1, io_channel.1, &debug_channel.0, symbols);
    spawn(move || {
        let (bus, _) = machine_info
            .create_bus(Arc::new(Mutex::new(TuiInputQueue::new())), &image)
            .expect("Must succeed");
        TuiHost::new(
            machine_info,
            bus,
            debug_channel.1,
            monitor_channel.0,
            io_channel.0,
        )
        .run(image.start)
        .expect("Must succeed");
    });
    ui.run();
    Ok(())
}
