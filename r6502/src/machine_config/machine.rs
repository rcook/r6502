use std::path::PathBuf;

use crate::machine_config::bus_device::BusDevice;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Machine {
    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(rename = "baseImage")]
    pub(crate) base_image_path: Option<PathBuf>,

    #[serde(rename = "busDevices")]
    pub(crate) bus_devices: Vec<BusDevice>,
}
