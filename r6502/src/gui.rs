use crate::{Ui, UiHost};
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
    let mut ui = Ui::new(monitor_channel.1, io_channel.1, debug_channel.0, symbols)?;
    spawn(move || {
        UiHost::new(debug_channel.1, monitor_channel.0, io_channel.0)
            .run(image)
            .expect("Must succeed")
    });
    ui.run();
    Ok(())
}
