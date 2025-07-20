use crate::InterfaceAdapter;
use r6502config::{BusDevice as BusDevice_config, BusDeviceType, CharSet};
use r6502cpu::{BusDevice, DeviceMapping, InterruptEvent, Ram, Rom};
use r6502lib::emulator::{BusEvent, IoChannel, OutputDevice};
use r6502snapshot::MemoryImage;
use std::sync::mpsc::Sender;

#[must_use]
pub fn map_io_device(
    bus_device: &BusDevice_config,
    output: Box<dyn OutputDevice>,
    io_channel: IoChannel,
    bus_tx: &Sender<BusEvent>,
    interrupt_tx: Sender<InterruptEvent>,
    char_set: CharSet,
) -> DeviceMapping {
    let device: Box<dyn BusDevice> = match bus_device.r#type {
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
        address_range: bus_device.address_range.clone(),
        device,
        offset: bus_device.offset,
    }
}

#[must_use]
pub fn map_memory_device(bus_device: &BusDevice_config, images: &[&MemoryImage]) -> DeviceMapping {
    let memory_slices = images
        .iter()
        .map(|image| image.slice(&bus_device.address_range))
        .collect();
    let device: Box<dyn BusDevice> = match bus_device.r#type {
        BusDeviceType::Pia | BusDeviceType::Via => unimplemented!(),
        BusDeviceType::Ram => Box::new(Ram::new(bus_device.address_range.len(), &memory_slices)),
        BusDeviceType::Rom => Box::new(Rom::new(bus_device.address_range.len(), &memory_slices)),
    };
    DeviceMapping {
        address_range: bus_device.address_range.clone(),
        device,
        offset: bus_device.offset,
    }
}
