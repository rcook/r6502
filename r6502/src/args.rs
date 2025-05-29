use clap::{ArgAction, Parser};
use clap_num::maybe_hex;
use std::path::PathBuf;

#[derive(Parser)]
pub(crate) struct Args {
    pub(crate) path: PathBuf,

    #[clap(long="origin", value_parser=maybe_hex::<u16>)]
    pub(crate) origin: Option<u16>,

    #[clap(long="start", value_parser=maybe_hex::<u16>)]
    pub(crate) start: Option<u16>,

    #[arg(
        help = "Launch in debugger [default]",
        long = "no-debug",
        default_value_t = true,
        action = ArgAction::SetFalse
    )]
    pub(crate) debug: bool,

    #[arg(
        help = "Do not launch in debugger",
        long = "debug",
        overrides_with = "debug"
    )]
    _no_debug: bool,
}
