use crate::text_ui::cursive_tui::CursiveTui;
use crate::text_ui::debug_options::DebugOptions;
use crate::text_ui::tui_host::TuiHost;
use anyhow::Result;
use r6502config::CharSet;
use r6502cpu::InterruptChannel;
use r6502cpu::symbols::MapFile;
use r6502lib::emulator::char_set_util::translate_out;
use r6502lib::emulator::{IoChannel, MachineInfo, MemoryImage, OutputDevice};
use r6502lib::messages::IoMessage;
use std::sync::mpsc::{Sender, channel};
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
    fn write(&mut self, char_set: &CharSet, value: u8) -> Result<()> {
        if let Some(value) = translate_out(char_set, value) {
            self.io_tx.send(IoMessage::WriteChar(value as char))?;
        }
        Ok(())
    }
}

pub fn run_text_ui(opts: &DebugOptions) -> Result<()> {
    let image = MemoryImage::from_file(&opts.path)?;
    let machine_info = match image.machine_tag() {
        Some(tag) => MachineInfo::find_by_tag(tag)?,
        None => MachineInfo::find_by_name(&opts.machine)?,
    };

    let map_file = MapFile::load(&opts.path)?;

    let debug_channel = channel();
    let monitor_channel = channel();
    let io_channel = channel();

    let tui_output = TuiOutput::new(io_channel.0.clone());

    let input_channel = IoChannel::new();
    let input_tx = input_channel.tx.clone();

    let interrupt_channel = InterruptChannel::new();

    let _handle = spawn(move || {
        let (bus, _) = machine_info
            .create_bus(
                Box::new(tui_output),
                input_channel,
                interrupt_channel.tx,
                &image,
            )
            .expect("Must succeed");
        bus.start();

        TuiHost::new(machine_info, bus, debug_channel.1, monitor_channel.0).run(&image);
    });

    let mut ui = CursiveTui::new(
        monitor_channel.1,
        io_channel.1,
        &debug_channel.0,
        &input_tx,
        map_file,
    );
    ui.run();

    // TBD: Signal to thread to shut down etc. by extending DebugMessage with a shutdown message
    //if handle.join().is_err() {
    //    bail!("thread panicked: see r6502.log for info")
    //}

    Ok(())
}
