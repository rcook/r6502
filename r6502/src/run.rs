use crate::{initialize_vm, Args, Ui, UiHost};
use anyhow::Result;
use clap::Parser;
use r6502lib::{
    DummyMonitor, Image, Monitor, SymbolInfo, TracingMonitor, VmBuilder, OSHALT, OSWRCH,
};
use std::sync::mpsc::channel;
use std::thread::spawn;

pub(crate) fn run() -> Result<()> {
    let args = Args::parse();
    if args.debug {
        run_ui_host(&args)?;
    } else {
        run_cli_host(&args)?;
    }
    Ok(())
}

fn run_ui_host(args: &Args) -> Result<()> {
    let image = Image::load(&args.path, args.origin, args.start)?;
    let symbols = SymbolInfo::load(&args.path)?;
    let debug_channel = channel();
    let monitor_channel = channel();
    let io_channel = channel();
    let mut ui = Ui::new(monitor_channel.1, io_channel.1, debug_channel.0, symbols)?;
    spawn(move || UiHost::new(debug_channel.1, monitor_channel.0, io_channel.0).run(image));
    ui.run();
    Ok(())
}

fn run_cli_host(args: &Args) -> Result<()> {
    let monitor: Box<dyn Monitor> = if args.trace {
        Box::new(TracingMonitor::default())
    } else {
        Box::new(DummyMonitor)
    };

    let mut vm = VmBuilder::default().monitor(monitor).build()?;

    let image = Image::load(&args.path, args.origin, args.start)?;
    let (os, rts) = initialize_vm(&mut vm, &image)?;

    loop {
        while vm.step() {}

        match os.is_os_vector_brk(&vm) {
            Some(OSHALT) => {
                break;
            }
            Some(OSWRCH) => {
                print!("{}", vm.s.reg.a as char);
                os.return_from_os_vector_brk(&mut vm, &rts);
            }
            _ => todo!(),
        }
    }

    Ok(())
}
