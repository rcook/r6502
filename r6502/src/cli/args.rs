use clap::{Parser, Subcommand};
use clap_num::maybe_hex;
use path_absolutize::Absolutize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(name = "debug", about = "Debug program", value_parser = parse_absolute_path)]
    Debug {
        path: PathBuf,

        #[arg(long = "load", value_parser = maybe_hex::<u16>)]
        load: Option<u16>,

        #[arg(long = "start", value_parser = maybe_hex::<u16>)]
        start: Option<u16>,
    },

    #[command(name = "run", about = "Run program")]
    Run(RunOptions),

    #[command(name = "validate", about = "Run validation suite")]
    Validate {
        #[arg(required = true, value_parser = parse_absolute_path)]
        report_path: PathBuf,

        #[arg(long = "filter")]
        filter: Option<String>,
    },

    #[command(
        name = "validate-json",
        about = "Run validation scenario in JSON format"
    )]
    ValidateJson {
        #[arg(required = true)]
        json: String,
    },
}

#[derive(Debug, Parser)]
pub(crate) struct RunOptions {
    #[arg(value_parser = parse_absolute_path)]
    pub(crate) path: PathBuf,

    #[arg(long = "load", value_parser = maybe_hex::<u16>)]
    pub(crate) load: Option<u16>,

    #[arg(long = "start", value_parser = maybe_hex::<u16>)]
    pub(crate) start: Option<u16>,

    #[arg(help = "Trace execution", long = "trace", default_value_t = false)]
    pub(crate) trace: bool,

    #[arg(help = "Report cycles", long = "cycles", default_value_t = false)]
    pub(crate) cycles: bool,

    #[arg(
        help = "Start execution from RESET vector",
        long = "reset",
        default_value_t = false
    )]
    pub(crate) reset: bool,

    #[arg(help = "Stop after given number of cycles", long = "stop-after")]
    pub(crate) stop_after: Option<u32>,

    #[arg(
        help = "Machine hint if machine tag not in image header",
        long = "machine",
        short = 'm'
    )]
    pub(crate) machine: Option<String>,
}

fn parse_absolute_path(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}
