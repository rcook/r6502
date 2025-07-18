use crate::{HostHookType, deserialize_word};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct HostHook {
    #[serde(rename = "type")]
    pub r#type: HostHookType,

    #[serde(rename = "address", deserialize_with = "deserialize_word")]
    pub addr: u16,
}
