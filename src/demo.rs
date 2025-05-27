use crate::{run_vm, Args, ProgramInfo, UI};
use anyhow::Result;
use clap::Parser;
use std::sync::mpsc::channel;
use std::thread::spawn;

pub(crate) fn demo() -> Result<()> {
    let args = Args::parse();

    let program_info = Some(ProgramInfo::new(&args.path, args.start));

    let cpu_channel = channel();
    let ui_channel = channel();
    let mut ui = UI::new(ui_channel.1, cpu_channel.0)?;

    spawn(move || {
        run_vm(cpu_channel.1, ui_channel.0, program_info).unwrap();
    });

    ui.run();

    Ok(())
}
