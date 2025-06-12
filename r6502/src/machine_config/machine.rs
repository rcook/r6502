use crate::machine_config::bus_device::BusDevice;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Machine {
    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(rename = "busDevices")]
    pub(crate) bus_devices: Vec<BusDevice>,
}
