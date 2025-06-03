use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(name = "run", about = "Run validation suite")]
    Run {
        #[arg(long = "filter")]
        filter: Option<String>,
    },

    #[command(name = "run-json", about = "Run scenario provided on command line")]
    RunJson {
        #[arg(required = true)]
        json: String,
    },
}
