use crate::machine_config::machine::Machine;
use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Machines {
    #[serde(rename = "defaultMachine")]
    pub default_machine: String,

    #[serde(rename = "machines")]
    pub machines: Vec<Machine>,
}

impl Machines {
    pub fn read(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }
}
