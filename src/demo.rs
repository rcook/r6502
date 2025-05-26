use crate::{Controller, ProgramInfo};
use anyhow::Result;
use std::path::Path;

pub(crate) fn demo() -> Result<()> {
    let mut controller = Controller::new()?;
    controller.run(Some(ProgramInfo::new(
        Path::new("examples\\Main.bin"),
        0x2000,
    )))?;
    Ok(())
}
