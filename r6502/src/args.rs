use clap::Parser;
use clap_num::maybe_hex;
use std::path::PathBuf;

#[derive(Parser)]
pub(crate) struct Args {
    pub(crate) path: PathBuf,

    #[clap(long="origin", value_parser=maybe_hex::<u16>)]
    pub(crate) origin: Option<u16>,

    #[clap(long="start", value_parser=maybe_hex::<u16>)]
    pub(crate) start: Option<u16>,

    #[arg(help = "Launch in debugger", long = "debug", default_value_t = false)]
    pub(crate) debug: bool,

    #[arg(help = "Trace execution [", long = "trace", default_value_t = false)]
    pub(crate) trace: bool,
}
