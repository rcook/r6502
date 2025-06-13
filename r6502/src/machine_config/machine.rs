use crate::machine_config::bus_device::BusDevice;
use r6502lib::deserialization::deserialize_word_opt;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Machine {
    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(rename = "baseImage")]
    pub(crate) base_image_path: Option<PathBuf>,

    #[serde(
        rename = "haltAddress",
        deserialize_with = "deserialize_word_opt",
        default
    )]
    pub(crate) halt_addr: Option<u16>,

    #[serde(
        rename = "writeCharAddress",
        deserialize_with = "deserialize_word_opt",
        default
    )]
    pub(crate) write_char_addr: Option<u16>,

    #[serde(rename = "busDevices")]
    pub(crate) bus_devices: Vec<BusDevice>,
}
