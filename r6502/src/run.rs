use crate::args::Command;
use crate::{Args, Ui, UiHost};
use anyhow::{anyhow, Result};
use clap::Parser;
use r6502lib::{
    DummyMonitor, Image, Monitor, Opcode, OsBuilder, SymbolInfo, TracingMonitor, VmBuilder,
    MOS_6502, OSHALT, OSWRCH,
};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::spawn;

pub(crate) fn run() -> Result<()> {
    match Args::parse().command {
        Command::Run {
            path,
            origin,
            start,
            trace,
        } => run_cli_host(&path, origin, start, trace)?,
        Command::Debug {
            path,
            origin,
            start,
        } => run_ui_host(&path, origin, start)?,
    }
    Ok(())
}

fn run_ui_host(path: &Path, origin: Option<u16>, start: Option<u16>) -> Result<()> {
    let image = Image::load(path, origin, start)?;
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

fn run_cli_host(path: &Path, origin: Option<u16>, start: Option<u16>, trace: bool) -> Result<()> {
    let monitor: Box<dyn Monitor> = if trace {
        Box::new(TracingMonitor::default())
    } else {
        Box::new(DummyMonitor)
    };

    let mut vm = VmBuilder::default().monitor(monitor).build()?;
    let image = Image::load(path, origin, start)?;
    vm.s.memory.load(&image);
    vm.s.reg.pc = image.start;

    let os = OsBuilder::default().build()?;
    os.load_into_vm(&mut vm);

    let rti = MOS_6502
        .get_op_info(&Opcode::Rti)
        .ok_or_else(|| anyhow!("RTI must exist"))?
        .clone();

    loop {
        while vm.step() {}

        match os.is_os_vector(&vm) {
            Some(OSHALT) => {
                break;
            }
            Some(OSWRCH) => {
                print!("{}", vm.s.reg.a as char);
                rti.execute_no_operand(&mut vm.s);
            }
            _ => todo!(),
        }
    }

    Ok(())
}
