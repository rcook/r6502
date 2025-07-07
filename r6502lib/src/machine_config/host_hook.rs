use crate::emulator::deserialization::deserialize_word;
use crate::machine_config::host_hook_type::HostHookType;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct HostHook {
    #[serde(rename = "type")]
    pub r#type: HostHookType,

    #[serde(rename = "address", deserialize_with = "deserialize_word")]
    pub addr: u16,
}
