use crate::machine_config::bus_device::BusDevice;
use r6502lib::emulator::deserialization::{deserialize_machine_tag, deserialize_word_opt};
use r6502lib::emulator::MachineTag;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize)]
pub struct Machine {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "tag", deserialize_with = "deserialize_machine_tag")]
    pub tag: MachineTag,

    #[serde(rename = "baseImage")]
    pub base_image_path: Option<PathBuf>,

    #[serde(
        rename = "haltAddress",
        deserialize_with = "deserialize_word_opt",
        default
    )]
    pub halt_addr: Option<u16>,

    #[serde(
        rename = "writeCharAddress",
        deserialize_with = "deserialize_word_opt",
        default
    )]
    pub write_char_addr: Option<u16>,

    #[serde(rename = "busDevices")]
    pub bus_devices: Vec<BusDevice>,
}
