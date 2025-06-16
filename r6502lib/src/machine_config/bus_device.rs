use crate::emulator::deserialization::deserialize_word;
use crate::emulator::{
    AddressRange, BusDevice as _BusDevice, BusEvent, DeviceMapping, Image, OutputDevice, Pia, Ram,
    Rom,
};
use crate::machine_config::bus_device_type::BusDeviceType;
use cursive::backends::crossterm::crossterm::event::Event;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use std::sync::mpsc::{Receiver, Sender};

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
        event_rx: Receiver<Event>,
        bus_tx: &Sender<BusEvent>,
    ) -> DeviceMapping {
        let device: Box<dyn _BusDevice> = match self.r#type {
            BusDeviceType::Pia => Box::new(Pia::new(output, event_rx, bus_tx.clone())),
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
            BusDeviceType::Pia => unimplemented!(),
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
