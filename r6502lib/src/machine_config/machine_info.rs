use crate::emulator::{
    Bus, BusEvent, Image, MachineTag, OutputDevice, PiaChannel, NULL_MACHINE_TAG,
};
use crate::machine_config::bus_device_type::BusDeviceType;
use crate::machine_config::machine::Machine;
use crate::machine_config::machines::Machines;
use anyhow::{anyhow, bail, Result};
use dirs::config_dir;
use path_absolutize::Absolutize;
use std::env::current_exe;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};

#[derive(Debug)]
pub struct MachineInfo {
    pub config_dir: PathBuf,
    pub machine: Machine,
}

impl MachineInfo {
    pub fn find_by_tag(tag: MachineTag) -> Result<Self> {
        let (machines, config_dir) = Self::read_machines()?;
        let machine = machines
            .machines
            .iter()
            .find(|m| m.tag == tag)
            .ok_or_else(|| anyhow!("No such machine"))?
            .clone();

        Ok(Self {
            config_dir,
            machine,
        })
    }

    pub fn find_by_name(name: &Option<String>) -> Result<Self> {
        let (machines, config_dir) = Self::read_machines()?;
        let name = name.as_ref().unwrap_or(&machines.default_machine);
        let machine = machines
            .machines
            .iter()
            .find(|m| m.name == *name)
            .ok_or_else(|| anyhow!("No such machine"))?
            .clone();

        Ok(Self {
            config_dir,
            machine,
        })
    }

    pub fn create_bus(
        &self,
        output: Box<dyn OutputDevice>,
        pia_channel: PiaChannel,
        image: &Image,
    ) -> Result<(Bus, Receiver<BusEvent>)> {
        let mut images = Vec::new();

        let mut base_image = None;

        if let Some(p) = self.machine.base_image_path.as_ref() {
            let base_image_path = p
                .absolutize_from(&self.config_dir)
                .map_err(|e| anyhow!(e))?
                .to_path_buf();
            base_image = Some(Image::from_file(&base_image_path)?);
            images.push(base_image.as_ref().expect("Must be valid"));
        }
        images.push(image);

        let (bus_tx, bus_rx) = channel();

        let mut io_devices = Vec::new();
        let mut memory_devices = Vec::new();
        for d in &self.machine.bus_devices {
            match d.r#type {
                BusDeviceType::Pia => io_devices.push(d),
                BusDeviceType::Ram | BusDeviceType::Rom => memory_devices.push(d),
            }
        }

        if io_devices.len() > 1 {
            bail!("Only one I/O bus device allowed")
        }

        let mut mappings = Vec::with_capacity(self.machine.bus_devices.len());

        if let Some(d) = io_devices.first() {
            mappings.push(d.map_io_device(output, pia_channel, &bus_tx));
        }

        for d in memory_devices {
            mappings.push(d.map_memory_device(&images));
        }

        drop(base_image);

        let bus = Bus::new(image.machine_tag().unwrap_or(NULL_MACHINE_TAG), mappings);
        Ok((bus, bus_rx))
    }

    fn get_config_dir(bin_path: &Path) -> Result<PathBuf> {
        fn user_config_dir() -> Result<PathBuf> {
            Ok(config_dir()
                .ok_or_else(|| anyhow!("Could not get configuration directory"))?
                .join("r6502"))
        }

        let p0 = bin_path
            .parent()
            .ok_or_else(|| anyhow!("Cannot get parent directory from {}", bin_path.display()))?;
        let d = p0.file_name().and_then(OsStr::to_str);
        if d != Some("debug") && d != Some("release") {
            return user_config_dir();
        }

        let p1 = p0
            .parent()
            .ok_or_else(|| anyhow!("Cannot get parent directory from {}", p0.display()))?;
        if p1.file_name().and_then(OsStr::to_str) != Some("target") {
            return user_config_dir();
        }

        let p2 = p1
            .parent()
            .ok_or_else(|| anyhow!("Cannot get parent directory from {}", p1.display()))?;

        Ok(p2.join("config"))
    }

    fn read_machines() -> Result<(Machines, PathBuf)> {
        let bin_path = current_exe()?;
        let config_dir = Self::get_config_dir(&bin_path)?;
        let config_path = config_dir.join("machines.json");
        if !config_path.is_file() {
            bail!(
                "Could not find configuration file at {}",
                config_path.display()
            )
        }

        Ok((Machines::read(&config_path)?, config_dir))
    }
}
