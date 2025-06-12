use crate::deserialization::deserialize_word;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct SymbolInfo {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "value",
        deserialize_with = "deserialize_word",
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

/*
fn serialize_value<S>(value: u16, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("${:04X}", value))
}
*/
