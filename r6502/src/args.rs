use clap::{Parser, Subcommand};
use clap_num::maybe_hex;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(name = "run", about = "Run program")]
    Run {
        path: PathBuf,

        #[arg(long = "origin", value_parser = maybe_hex::<u16>)]
        origin: Option<u16>,

        #[arg(long = "start", value_parser = maybe_hex::<u16>)]
        start: Option<u16>,

        #[arg(help = "Trace execution", long = "trace", default_value_t = false)]
        trace: bool,
    },

    #[command(name = "debug", about = "Debug program")]
    Debug {
        path: PathBuf,

        #[arg(long = "origin", value_parser = maybe_hex::<u16>)]
        origin: Option<u16>,

        #[arg(long = "start", value_parser = maybe_hex::<u16>)]
        start: Option<u16>,
    },

    #[command(name = "run-validation", about = "Run validation suite")]
    RunValidation {
        #[arg(long = "filter")]
        filter: Option<String>,
    },
}
