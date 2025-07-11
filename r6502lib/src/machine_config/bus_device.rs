use crate::emulator::deserialization::deserialize_word;
use crate::emulator::{
    AddressRange, BusDevice as _BusDevice, BusEvent, DeviceMapping, Image, InterfaceAdapter,
    InterruptEvent, IoChannel, OutputDevice, Ram, Rom,
};
use crate::machine_config::bus_device_type::BusDeviceType;
use crate::machine_config::CharSet;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use std::sync::mpsc::Sender;

#[derive(Clone, Debug, Deserialize)]
pub struct BusDevice {
    #[serde(rename = "type")]
    pub r#type: BusDeviceType,

    #[serde(
        rename = "addressRange",
        deserialize_with = "deserialize_address_range"
    )]
    pub address_range: AddressRange,

    #[serde(rename = "offset", deserialize_with = "deserialize_word")]
    pub offset: u16,
}

impl BusDevice {
    pub fn map_io_device(
        &self,
        output: Box<dyn OutputDevice>,
        io_channel: IoChannel,
        bus_tx: &Sender<BusEvent>,
        interrupt_tx: Sender<InterruptEvent>,
        char_set: CharSet,
    ) -> DeviceMapping {
        let device: Box<dyn _BusDevice> = match self.r#type {
            BusDeviceType::Pia | BusDeviceType::Via => Box::new(InterfaceAdapter::new(
                output,
                io_channel,
                bus_tx.clone(),
                interrupt_tx,
                char_set,
            )),
            BusDeviceType::Ram | BusDeviceType::Rom => unimplemented!(),
        };
        DeviceMapping {
            address_range: self.address_range.clone(),
            device,
            offset: self.offset,
        }
    }

    pub fn map_memory_device(&self, images: &[&Image]) -> DeviceMapping {
        let image_slices = images
            .iter()
            .map(|image| image.slice(&self.address_range))
            .collect();
        let device: Box<dyn _BusDevice> = match self.r#type {
            BusDeviceType::Pia | BusDeviceType::Via => unimplemented!(),
            BusDeviceType::Ram => Box::new(Ram::new(self.address_range.len(), &image_slices)),
            BusDeviceType::Rom => Box::new(Rom::new(self.address_range.len(), &image_slices)),
        };
        DeviceMapping {
            address_range: self.address_range.clone(),
            device,
            offset: self.offset,
        }
    }
}

fn deserialize_address_range<'de, D>(deserializer: D) -> Result<AddressRange, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(SerdeError::custom)
}
