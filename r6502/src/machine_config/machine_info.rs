use crate::machine_config::machine::Machine;
use crate::machine_config::machines::Machines;
use anyhow::{anyhow, bail, Result};
use dirs::config_dir;
use path_absolutize::Absolutize;
use r6502lib::{Bus, BusEvent, Image};
use std::env::current_exe;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};

pub(crate) struct MachineInfo {
    pub(crate) config_dir: PathBuf,
    pub(crate) machine: Machine,
}

impl MachineInfo {
    pub(crate) fn load(machine: &Option<String>) -> Result<Self> {
        let bin_path = current_exe()?;
        let config_dir = Self::get_config_dir(&bin_path)?;

        let config_path = config_dir.join("machines.json");

        if !config_path.is_file() {
            bail!(
                "Could not find configuration file at {}",
                config_path.display()
            )
        }

        let machines = Machines::read(&config_path)?;

        let machine_name = machine.as_ref().unwrap_or(&machines.default_machine);

        let machine = machines
            .machines
            .iter()
            .find(|m| m.name == *machine_name)
            .ok_or_else(|| anyhow!("No such machine"))?
            .clone();

        Ok(Self {
            config_dir,
            machine,
        })
    }

    pub(crate) fn create_bus(&self, image: &Image) -> Result<(Bus, Receiver<BusEvent>)> {
        let mut images = Vec::new();

        #[allow(unused_assignments)]
        let mut base_image = None;

        if let Some(p) = self.machine.base_image_path.as_ref() {
            let base_image_path = p
                .absolutize_from(&self.config_dir)
                .map_err(|e| anyhow!(e))?
                .to_path_buf();
            base_image = Some(Image::load(&base_image_path, None, None, None)?);
            images.push(base_image.as_ref().expect("Must be valid"));
        }
        images.push(image);

        let (bus_tx, bus_rx) = channel();
        let mappings = self
            .machine
            .bus_devices
            .iter()
            .map(|d| d.create_device_mapping(&bus_tx, &images))
            .collect();
        let bus = Bus::new(mappings);
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
        if p0.file_name().and_then(OsStr::to_str) != Some("debug") {
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
}
