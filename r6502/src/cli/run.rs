use crate::cli::{Args, Command};
use crate::terminal::run_terminal;
use crate::tui::run_gui;
use anyhow::Result;
use clap::Parser;

pub(crate) fn run() -> Result<()> {
    match Args::parse().command {
        Command::Run(opts) => run_terminal(&opts)?,
        Command::Debug { path, load, start } => run_gui(&path, load, start)?,
    }
    Ok(())
}
