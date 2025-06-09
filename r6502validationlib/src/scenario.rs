use crate::{Cycle, State};
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Deserialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Scenario {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "initial")]
    pub initial: State,

    #[serde(rename = "final")]
    pub r#final: State,

    #[allow(unused)]
    #[serde(rename = "cycles")]
    pub cycles: Vec<Cycle>,
}

impl Scenario {
    pub fn from_json_file(path: &Path) -> Result<Vec<Self>> {
        let file = File::open(path)?;
        serde_json::from_reader(file).map_err(|e| anyhow!(e))
    }

    pub fn from_json(s: &str) -> Result<Self> {
        serde_json::from_str(s).map_err(|e| anyhow!(e))
    }

    pub fn read_rkyv(path: &Path) -> Result<Vec<Self>> {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        _ = file.read_to_end(&mut bytes)?;
        Ok(rkyv::from_bytes::<_, rancor::Error>(&bytes)?)
    }

    pub fn write_rkyv(path: &Path, scenarios: &Vec<Scenario>) -> Result<()> {
        let bytes = rkyv::to_bytes::<rancor::Error>(scenarios)?;
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Scenario: {}", self.name)?;
        write!(f, "Initial:\n{}", self.initial)?;
        write!(f, "Final:\n{}", self.r#final)
    }
}
