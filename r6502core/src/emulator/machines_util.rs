use anyhow::Result;
use r6502config::Machines;
use std::fs::File;
use std::path::Path;

pub fn read(path: &Path) -> Result<Machines> {
    let file = File::open(path)?;
    Ok(serde_json::from_reader(file)?)
}
