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

        #[arg(long = "load", value_parser = maybe_hex::<u16>)]
        load: Option<u16>,

        #[arg(long = "start", value_parser = maybe_hex::<u16>)]
        start: Option<u16>,

        #[arg(help = "Trace execution", long = "trace", default_value_t = false)]
        trace: bool,

        #[arg(help = "Report cycles", long = "cycles", default_value_t = false)]
        cycles: bool,

        #[arg(
            help = "Start execution from RESET vector",
            long = "reset",
            default_value_t = false
        )]
        reset: bool,

        #[arg(
            help = "Load my fake OS into memory",
            long = "fake-os",
            default_value_t = false
        )]
        fake_os: bool,
    },

    #[command(name = "debug", about = "Debug program")]
    Debug {
        path: PathBuf,

        #[arg(long = "load", value_parser = maybe_hex::<u16>)]
        load: Option<u16>,

        #[arg(long = "start", value_parser = maybe_hex::<u16>)]
        start: Option<u16>,
    },
}
