use clap::{Parser, Subcommand};
use path_absolutize::Absolutize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(name = "import", about = "Import JSON scenarios")]
    Import {
        #[arg(required = true, value_parser = parse_absolute_path)]
        input_dir: PathBuf,

        #[arg(required = true, value_parser = parse_absolute_path)]
        output_dir: PathBuf,
    },

    #[command(name = "run", about = "Run validation suite")]
    Run {
        #[arg(required = true, value_parser = parse_absolute_path)]
        report_path: PathBuf,

        #[arg(long = "filter")]
        filter: Option<String>,
    },

    #[command(name = "run-json", about = "Run scenario provided on command line")]
    RunJson {
        #[arg(required = true)]
        json: String,
    },
}

fn parse_absolute_path(s: &str) -> Result<PathBuf, String> {
    PathBuf::from(s)
        .absolutize()
        .map_err(|_| String::from("invalid path"))
        .map(|x| x.to_path_buf())
}
