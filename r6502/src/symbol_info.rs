use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serializer};

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub(crate) struct SymbolInfo {
    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(
        rename = "value",
        deserialize_with = "deserialize_value",
        serialize_with = "serialize_value"
    )]
    pub(crate) value: u16,

    #[serde(
        rename = "source_location",
        alias = "sourceLocation",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub(crate) source_location: Option<String>,
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

#[allow(unused)]
fn serialize_value<S>(value: u16, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("${:04X}", value))
}
