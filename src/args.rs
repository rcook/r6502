use clap::Parser;
use clap_num::maybe_hex;
use std::path::PathBuf;

#[derive(Parser)]
pub(crate) struct Args {
    pub(crate) path: PathBuf,

    #[clap(value_parser=maybe_hex::<u16>)]
    pub(crate) start: u16,
}
