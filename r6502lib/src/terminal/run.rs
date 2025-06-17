use crate::emulator::{Cpu, Image, Monitor, PiaChannel, Snapshot, TracingMonitor};
use crate::machine_config::MachineInfo;
use crate::run_options::RunOptions;
use crate::terminal::{show_image_info, Runner, TerminalChannel, TerminalOutput};
use anyhow::Result;
use std::path::Path;

pub fn run(opts: &RunOptions) -> Result<()> {
    let image = Image::from_file(&opts.path)?;
    let machine_info = match image.machine_tag() {
        Some(tag) => MachineInfo::find_by_tag(tag)?,
        None => MachineInfo::find_by_name(&opts.machine)?,
    };

    let terminal_channel = TerminalChannel::new();
    let pia_channel = PiaChannel::new();
    let pia_tx = pia_channel.tx.clone();

    let (bus, bus_rx) = machine_info.create_bus(Box::new(TerminalOutput), pia_channel, &image)?;
    bus.start();

    let start = if opts.reset {
        bus.load_reset_unsafe()
    } else {
        image.start().or(opts.start).unwrap_or_default()
    };

    if opts.trace {
        show_image_info(opts, &image, start);
    }

    let monitor: Option<Box<dyn Monitor>> = if opts.trace {
        Some(Box::new(TracingMonitor::default()))
    } else {
        None
    };

    let mut cpu = Cpu::new(bus.view(), monitor);
    cpu.reg.pc = start;

    Runner {
        cpu: &mut cpu,
        bus_rx,
        pia_tx,
        terminal_channel,
        stop_after: opts.stop_after,
        machine_info,
        bus: &bus,
        cycles: opts.cycles,
    }
    .run()
}

pub fn run_from_snapshot(path: &Path) -> Result<()> {
    let snapshot = Snapshot::read(path)?;
    let machine_info = MachineInfo::find_by_tag(snapshot.header.machine_tag())?;
    //let (bus, bus_rx) = machine_info.create_bus(Box::new(TerminalOutput), pia_channel, &image)?;
    todo!("{machine_info:?}");
}
