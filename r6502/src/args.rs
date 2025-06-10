use clap::{Parser, Subcommand, ValueEnum};
use clap_num::maybe_hex;
use path_absolutize::Absolutize;
use r6502lib::MachineType;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(name = "run", about = "Run program")]
    Run(RunOptions),

    #[command(name = "debug", about = "Debug program", value_parser = parse_absolute_path)]
    Debug {
        path: PathBuf,

        #[arg(long = "load", value_parser = maybe_hex::<u16>)]
        load: Option<u16>,

        #[arg(long = "start", value_parser = maybe_hex::<u16>)]
        start: Option<u16>,
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
        help = "OS emulation mode",
        long = "emu",
        value_enum,
        default_value_t = Emulation::None
    )]
    pub(crate) emulation: Emulation,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum Emulation {
    #[clap(name = "none")]
    None,

    #[clap(name = "sim65")]
    Sim6502,

    #[clap(name = "acorn")]
    AcornStyle,

    #[clap(name = "apple1")]
    Apple1Style,
}

impl From<Emulation> for MachineType {
    fn from(value: Emulation) -> Self {
        match value {
            Emulation::None => MachineType::None,
            Emulation::Sim6502 => MachineType::Sim6502,
            Emulation::AcornStyle => MachineType::Acorn,
            Emulation::Apple1Style => MachineType::Apple1,
        }
    }
}

fn parse_absolute_path(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}
