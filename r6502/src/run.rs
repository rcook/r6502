use crate::args::Command;
use crate::{run_gui, run_terminal, Args};
use anyhow::Result;
use clap::Parser;

pub(crate) fn run() -> Result<()> {
    match Args::parse().command {
        Command::Run(opts) => run_terminal(&opts)?,
        Command::Debug { path, load, start } => run_gui(&path, load, start)?,
    }
    Ok(())
}
