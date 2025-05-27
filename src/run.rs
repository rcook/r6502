use crate::{run_vm, Args, ProgramInfo, UI};
use anyhow::Result;
use clap::Parser;
use std::sync::mpsc::channel;
use std::thread::spawn;

pub(crate) fn run() -> Result<()> {
    let args = Args::parse();
    let program_info = Some(ProgramInfo::new(&args.path, args.start));
    let debug_channel = channel();
    let status_channel = channel();
    let mut ui = UI::new(status_channel.1, debug_channel.0)?;
    spawn(move || {
        run_vm(debug_channel.1, status_channel.0, program_info).expect("Must succeed");
    });
    ui.run();
    Ok(())
}
