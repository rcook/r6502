use crate::args::Command;
use crate::{run_gui, run_terminal, Args};
use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use simple_logging::log_to_file;

pub(crate) fn run() -> Result<()> {
    log_to_file("r6502.log", LevelFilter::Info)?;
    match Args::parse().command {
        Command::Run(opts) => run_terminal(&opts)?,
        Command::Debug { path, load, start } => run_gui(&path, load, start)?,
    }
    Ok(())
}
