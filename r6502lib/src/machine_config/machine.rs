use crate::emulator::deserialization::{deserialize_machine_tag, deserialize_word_opt};
use crate::emulator::MachineTag;
use crate::machine_config::bus_device::BusDevice;
use crate::machine_config::host_hook::HostHook;
use crate::machine_config::output_device_type::OutputDeviceType;
use crate::machine_config::CharSet;
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

    #[serde(rename = "hostHook")]
    pub host_hook: Option<HostHook>,

    #[serde(rename = "outputDeviceType", default)]
    pub output_device_type: OutputDeviceType,

    #[serde(rename = "busDevices")]
    pub bus_devices: Vec<BusDevice>,

    #[serde(rename = "charSet", default)]
    pub char_set: CharSet,
}
