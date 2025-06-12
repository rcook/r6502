use crate::util::make_word;
use crate::{
    AddressRange, BusEvent, BusView, DeviceMapping, Image, MachineType, IRQ, MEMORY_SIZE, NMI,
    RESET,
};
use anyhow::{bail, Result};
use std::sync::mpsc::{channel, Sender};

const UNMAPPED_VALUE: u8 = 0xff;

// Represents the address bus and attached memory-mapped devices including RAM/ROM/PIA
pub struct Bus {
    machine_type: MachineType,
    mappings: Vec<DeviceMapping>,
}

impl Default for Bus {
    fn default() -> Self {
        let (bus_tx, _) = channel();
        Self::configure_for(MachineType::None, &bus_tx)
    }
}

impl Bus {
    #[must_use]
    pub fn configure_for(machine_type: MachineType, bus_tx: &Sender<BusEvent>) -> Self {
        let mut descriptions = machine_type.get_device_descriptions();
        descriptions.sort_by(|a, b| a.address_range.start().cmp(&b.address_range.start()));

        let mappings = descriptions
            .into_iter()
            .map(|d| DeviceMapping {
                address_range: d.address_range,
                device: (d.device_fn)(bus_tx.clone()),
                offset: d.offset,
            })
            .collect();
        Self::new(machine_type, mappings)
    }

    #[must_use]
    pub fn machine_type(&self) -> MachineType {
        self.machine_type
    }

    pub fn start(&self) {
        for mapping in &self.mappings {
            mapping.device.start();
        }
    }

    pub fn join(&self) {
        for mapping in &self.mappings {
            mapping.device.join();
        }
    }

    // Don't call this when more than one thread is concurrently accessing memory
    #[must_use]
    pub fn load_nmi_unsafe(&self) -> u16 {
        let lo = self.load(NMI);
        let hi = self.load(NMI + 1);
        make_word(hi, lo)
    }

    // Don't call this when more than one thread is concurrently accessing memory
    #[must_use]
    pub fn load_reset_unsafe(&self) -> u16 {
        let lo = self.load(RESET);
        let hi = self.load(RESET + 1);
        make_word(hi, lo)
    }

    // Don't call this when more than one thread is concurrently accessing memory
    #[must_use]
    pub fn load_irq_unsafe(&self) -> u16 {
        let lo = self.load(IRQ);
        let hi = self.load(IRQ + 1);
        make_word(hi, lo)
    }

    pub fn store_image(&self, image: &Image) -> Result<()> {
        let load = image.load;
        let limit = load as usize + image.values.len();
        if limit > MEMORY_SIZE {
            bail!(
                "Image size ${size:04X} starting at load address ${load:04X} is too big",
                size = image.values.len()
            );
        }

        // Also incredibly inefficient
        for (i, value) in image.values.iter().enumerate() {
            self.store(i as u16 + load, *value);
        }

        Ok(())
    }

    #[must_use]
    pub fn snapshot(&self, address_range: &AddressRange) -> Vec<u8> {
        let mut result = Vec::with_capacity(address_range.len());

        // Incredibly inefficient!
        for addr in address_range.start()..=address_range.end() {
            result.push(self.load(addr));
        }

        result
    }

    #[must_use]
    pub fn view(&self) -> BusView {
        BusView::new(self)
    }

    #[must_use]
    pub fn load(&self, addr: u16) -> u8 {
        match self.find_mapping(addr) {
            Some(mapping) => mapping.device.load(addr - mapping.offset),
            None => UNMAPPED_VALUE,
        }
    }

    pub fn store(&self, addr: u16, value: u8) {
        if let Some(mapping) = self.find_mapping(addr) {
            mapping.device.store(addr - mapping.offset, value);
        }
    }

    #[must_use]
    fn new(machine_type: MachineType, mut mappings: Vec<DeviceMapping>) -> Self {
        assert!(!AddressRange::overlapping(
            &mappings
                .iter()
                .map(|m| m.address_range.clone())
                .collect::<Vec<_>>()
        ));
        mappings.sort_by(|a, b| a.address_range.start().cmp(&b.address_range.start()));
        Self {
            machine_type,
            mappings,
        }
    }

    fn find_mapping(&self, addr: u16) -> Option<&DeviceMapping> {
        self.mappings
            .iter()
            .find(|&mapping| mapping.address_range.contains(addr))
    }
}

#[cfg(test)]
mod tests {
    use crate::bus::UNMAPPED_VALUE;
    use crate::{AddressRange, Bus, DeviceMapping, Image, MachineType, Ram};
    use anyhow::Result;

    #[test]
    fn load_no_device() {
        let bus = Bus::new(MachineType::None, Vec::new());
        assert_eq!(UNMAPPED_VALUE, bus.load(0x0000));
    }

    #[test]
    fn store_no_device() {
        let bus = Bus::new(MachineType::None, Vec::new());
        bus.store(0x0000, 0x00);
    }

    #[test]
    fn store_image() -> Result<()> {
        let mappings = vec![DeviceMapping {
            address_range: AddressRange::new(5, 7).expect("Must succeed"),
            device: Box::new(Ram::<3>::default()),
            offset: 5,
        }];
        let bus = Bus::new(MachineType::None, mappings);
        let bytes = (0..=255).cycle().skip(10).take(100).collect::<Vec<_>>();
        let image = Image::from_bytes(&bytes, None, None, None)?;
        assert_eq!(0x0000, image.load);

        bus.store_image(&image)?;

        for addr in 0..5 {
            assert_eq!(UNMAPPED_VALUE, bus.load(addr));
        }
        assert_eq!(15, bus.load(5));
        assert_eq!(16, bus.load(6));
        assert_eq!(17, bus.load(7));
        for addr in 8..200 {
            assert_eq!(UNMAPPED_VALUE, bus.load(addr));
        }
        Ok(())
    }
}
