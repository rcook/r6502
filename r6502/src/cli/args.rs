use clap::{Parser, Subcommand, ValueEnum};
use clap_num::maybe_hex;
use path_absolutize::Absolutize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(name = "debug", about = "Debug program", value_parser = parse_absolute_path)]
    Debug(DebugOptions),

    #[command(name = "run", about = "Run program")]
    Run(RunOptions),

    #[command(name = "test-gui")]
    TestGraphicsTerminal {
        #[clap(long = "font", value_enum, default_value_t = Font::Bedstead)]
        font: Font,
    },

    #[command(name = "test-tui")]
    TestTextTerminal,

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
pub struct DebugOptions {
    #[arg(value_parser = parse_absolute_path)]
    pub path: PathBuf,

    #[arg(long = "load", value_parser = maybe_hex::<u16>)]
    pub load: Option<u16>,

    #[arg(long = "start", value_parser = maybe_hex::<u16>)]
    pub start: Option<u16>,

    #[arg(
        help = "Machine hint if machine tag not in image header",
        long = "machine",
        short = 'm'
    )]
    pub machine: Option<String>,
}

impl From<DebugOptions> for r6502ui::text_ui::DebugOptions {
    fn from(value: DebugOptions) -> Self {
        Self {
            path: value.path,
            load: value.load,
            start: value.start,
            machine: value.machine,
        }
    }
}

#[derive(Debug, Parser)]
pub struct RunOptions {
    #[arg(value_parser = parse_absolute_path)]
    pub path: PathBuf,

    #[arg(long = "load", value_parser = maybe_hex::<u16>)]
    pub load: Option<u16>,

    #[arg(long = "start", value_parser = maybe_hex::<u16>)]
    pub start: Option<u16>,

    #[arg(
        help = "Trace execution to log",
        long = "trace",
        default_value_t = false
    )]
    pub trace: bool,

    #[arg(help = "Report cycles", long = "cycles", default_value_t = false)]
    pub cycles: bool,

    #[arg(help = "Stop after given number of cycles", long = "stop-after")]
    pub stop_after: Option<u64>,

    #[arg(
        help = "Machine hint if machine tag not in image header",
        long = "machine",
        short = 'm'
    )]
    pub machine: Option<String>,
}

impl From<RunOptions> for r6502ui::terminal_ui::RunOptions {
    fn from(value: RunOptions) -> Self {
        Self {
            path: value.path,
            load: value.load,
            start: value.start,
            trace: value.trace,
            cycles: value.cycles,
            stop_after: value.stop_after,
            machine: value.machine,
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Font {
    #[clap(name = "acorn")]
    Acorn,

    #[clap(name = "bedstead")]
    Bedstead,

    #[clap(name = "5050")]
    MullardSaa5050,
}

impl From<Font> for r6502vdu::font::Font {
    fn from(value: Font) -> Self {
        match value {
            Font::Acorn => Self::Acorn,
            Font::Bedstead => Self::Bedstead,
            Font::MullardSaa5050 => Self::MullardSaa5050,
        }
    }
}

fn parse_absolute_path(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}
