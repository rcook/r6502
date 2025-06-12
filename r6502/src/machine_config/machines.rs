use crate::machine_config::Machine;
use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub(crate) struct Machines {
    #[serde(rename = "defaultMachine")]
    pub(crate) default_machine: String,

    #[serde(rename = "machines")]
    pub(crate) machines: Vec<Machine>,
}

impl Machines {
    pub(crate) fn read(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }
}
