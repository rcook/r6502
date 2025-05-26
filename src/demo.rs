use crate::{Args, Controller, ProgramInfo};
use anyhow::Result;
use clap::Parser;

pub(crate) fn demo() -> Result<()> {
    let args = Args::parse();
    let mut controller = Controller::new()?;
    controller.run(Some(ProgramInfo::new(&args.path, args.start)))?;
    Ok(())
}
