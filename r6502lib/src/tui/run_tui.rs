use crate::debug_options::DebugOptions;
use crate::emulator::{Image, OutputDevice, PiaChannel};
use crate::machine_config::MachineInfo;
use crate::messages::IoMessage;
use crate::symbols::MapFile;
use crate::tui::cursive_tui::CursiveTui;
use crate::tui::tui_host::TuiHost;
use anyhow::Result;
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;

struct TuiOutput {
    io_tx: Sender<IoMessage>,
}

impl TuiOutput {
    const fn new(io_tx: Sender<IoMessage>) -> Self {
        Self { io_tx }
    }
}

impl OutputDevice for TuiOutput {
    fn write(&self, ch: char) -> Result<()> {
        _ = self.io_tx.send(IoMessage::WriteChar(ch));
        Ok(())
    }
}

pub fn run_tui(opts: &DebugOptions) -> Result<()> {
    let image = Image::load(&opts.path, opts.load, opts.start, None)?;
    let machine_info = match image.machine_tag {
        Some(tag) => MachineInfo::find_by_tag(tag)?,
        None => MachineInfo::find_by_name(&opts.machine)?,
    };

    let map_file = MapFile::load(&opts.path)?;

    let debug_channel = channel();
    let monitor_channel = channel();
    let io_channel = channel();

    let tui_output = TuiOutput::new(io_channel.0.clone());

    let input_channel = PiaChannel::new();
    let input_tx = input_channel.sender.clone();

    spawn(move || {
        let (bus, _) = machine_info
            .create_bus(Box::new(tui_output), input_channel, &image)
            .expect("Must succeed");
        bus.start();

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

    let mut ui = CursiveTui::new(
        monitor_channel.1,
        io_channel.1,
        &debug_channel.0,
        &input_tx,
        map_file,
    );
    ui.run();
    Ok(())
}
