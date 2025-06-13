use crate::machine_config::machines::Machines;
use anyhow::{anyhow, Result};
use r6502lib::{Bus, BusEvent, Image};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};

pub(crate) fn create_bus(image: &Image) -> Result<(Bus, Receiver<BusEvent>)> {
    let machines = Machines::read(Path::new("machines.json"))?;
    let machine = machines
        .machines
        .iter()
        .find(|m| m.name == machines.default_machine)
        .ok_or_else(|| anyhow!("No such machine"))?;
    let (bus_tx, bus_rx) = channel();
    let mappings = machine
        .bus_devices
        .iter()
        .map(|d| d.create_device_mapping(&bus_tx, image))
        .collect();
    let bus = Bus::new(mappings);
    Ok((bus, bus_rx))
}
