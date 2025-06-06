use crate::args::Command;
use crate::{Args, Ui, UiHost};
use anyhow::{anyhow, Result};
use clap::Parser;
use r6502lib::{
    DummyMonitor, Image, Monitor, Opcode, OsBuilder, SymbolInfo, TracingMonitor, VmBuilder,
    MOS_6502, OSHALT, OSWRCH, RESET,
};
use std::path::Path;
use std::process::exit;
use std::sync::mpsc::channel;
use std::thread::spawn;

pub(crate) fn run() -> Result<()> {
    match Args::parse().command {
        Command::Run {
            path,
            load,
            start,
            trace,
            cycles,
            reset,
            fake_os,
        } => run_cli_host(&path, load, start, trace, cycles, reset, fake_os)?,
        Command::Debug { path, load, start } => run_ui_host(&path, load, start)?,
    }
    Ok(())
}

fn run_ui_host(path: &Path, load: Option<u16>, start: Option<u16>) -> Result<()> {
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

fn run_cli_host(
    path: &Path,
    load: Option<u16>,
    start: Option<u16>,
    trace: bool,
    cycles: bool,
    reset: bool,
    fake_os: bool,
) -> Result<()> {
    let monitor: Box<dyn Monitor> = if trace {
        Box::new(TracingMonitor::default())
    } else {
        Box::new(DummyMonitor)
    };

    let mut vm = VmBuilder::default().monitor(monitor).build()?;
    let rti = MOS_6502
        .get_op_info(&Opcode::Rti)
        .ok_or_else(|| anyhow!("RTI must exist"))?
        .clone();

    let image = Image::load(path, load, start, None)?;
    if trace {
        println!("Image: {}", path.display());
        println!("  Format:                {:?}", image.format);
        println!("  Load address:          ${:04X}", image.load);
        println!("  Start address:         ${:04X}", image.start);
        println!("  Initial stack pointer: ${:02X}", image.sp);
        println!(
            "  Start from RESET     : {}",
            if reset { "yes" } else { "no" }
        );
    }

    vm.s.memory.load(&image);
    let os = if fake_os {
        let os = OsBuilder::default().build()?;
        os.load_into_vm(&mut vm);
        Some(os)
    } else {
        None
    };

    if reset {
        vm.s.reg.pc = vm.s.memory.fetch_word(RESET);
    } else {
        vm.s.reg.pc = image.start;
    }

    if let Some(os) = &os {
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
                _ => break,
            }
        }
    } else {
        while vm.step() {}
    }

    // Program hit BRK: return contents of A as exit code
    if cycles {
        println!("Total cycles: {}", vm.total_cycles);
    }
    exit(vm.s.reg.a as i32);
}
