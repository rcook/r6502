use anyhow::{anyhow, Result};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct SymbolInfo {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "value",
        deserialize_with = "deserialize_value",
        serialize_with = "serialize_value"
    )]
    pub value: u16,

    #[serde(
        rename = "source_location",
        alias = "sourceLocation",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub source_location: Option<String>,
}

impl SymbolInfo {
    pub fn load(image_path: &Path) -> Result<Vec<SymbolInfo>> {
        let mut file_name = image_path
            .file_name()
            .ok_or_else(|| anyhow!("could not get file name"))?
            .to_os_string();
        file_name.push(".json");
        let symbol_path = image_path
            .parent()
            .ok_or_else(|| anyhow!("could not get parent of path"))?
            .join(file_name);

        Ok(if symbol_path.is_file() {
            let file = File::open(symbol_path)?;
            serde_json::from_reader(file)?
        } else {
            Vec::new()
        })
    }
}

fn deserialize_value<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.strip_prefix('$') {
        Some(suffix) => u16::from_str_radix(suffix, 16).map_err(SerdeError::custom),
        None => s.parse().map_err(SerdeError::custom),
    }
}

/*
fn serialize_value<S>(value: u16, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("${:04X}", value))
}
*/
