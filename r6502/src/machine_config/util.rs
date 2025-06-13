use crate::machine_config::machines::Machines;
use anyhow::{anyhow, Result};
use dirs::config_dir;
use r6502lib::{Bus, BusEvent, Image};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::sync::mpsc::{channel, Receiver};

pub(crate) fn create_bus(
    image: &Image,
    machine: &Option<String>,
) -> Result<(Bus, Receiver<BusEvent>)> {
    let config_path = config_dir()
        .ok_or_else(|| anyhow!("Could not get configuration directory"))?
        .join("r6502")
        .join("machines.json");

    if !config_path.is_file() {
        create_dir_all(
            config_path
                .parent()
                .ok_or_else(|| anyhow!("Could not get parent directory"))?,
        )?;
        let mut file = File::create_new(&config_path)?;
        let s = include_str!("../../../machines.json");
        writeln!(file, "{s}")?;
    }

    let machines = Machines::read(&config_path)?;

    let machine_name = machine.as_ref().unwrap_or(&machines.default_machine);

    let machine = machines
        .machines
        .iter()
        .find(|m| m.name == *machine_name)
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
