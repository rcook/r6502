use crate::util::make_word;
use crate::{
    AddressRange, BusEvent, BusView, DeviceDescription, DeviceMapping, Image, MachineType, IRQ,
    NMI, RESET,
};
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
        Self::configure_for(MachineType::AllRam, &bus_tx, None)
    }
}

impl Bus {
    #[must_use]
    pub fn configure_for(
        machine_type: MachineType,
        bus_tx: &Sender<BusEvent>,
        image: Option<&Image>,
    ) -> Self {
        Self::new(
            machine_type,
            bus_tx,
            machine_type.get_device_descriptions(),
            image,
        )
    }

    #[must_use]
    pub const fn machine_type(&self) -> MachineType {
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
    pub const fn view(&self) -> BusView {
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
    pub(crate) fn new(
        machine_type: MachineType,
        bus_tx: &Sender<BusEvent>,
        mut descriptions: Vec<DeviceDescription>,
        image: Option<&Image>,
    ) -> Self {
        assert!(!AddressRange::overlapping(
            &descriptions
                .iter()
                .map(|m| m.address_range.clone())
                .collect::<Vec<_>>()
        ));
        descriptions.sort_by(|a, b| a.address_range.start().cmp(&b.address_range.start()));

        let mappings = descriptions
            .into_iter()
            .map(|d| {
                let slice = image.map(|i| i.slice(&d.address_range));
                DeviceMapping {
                    address_range: d.address_range,
                    device: (d.device_fn)(bus_tx.clone(), slice),
                    offset: d.offset,
                }
            })
            .collect();

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
    use crate::{AddressRange, Bus, DeviceDescription, Image, MachineType, Ram};
    use anyhow::Result;
    use std::sync::mpsc::channel;

    #[test]
    fn load_no_device() {
        let bus_channel = channel();
        let bus = Bus::new(MachineType::AllRam, &bus_channel.0, Vec::new(), None);
        assert_eq!(UNMAPPED_VALUE, bus.load(0x0000));
    }

    #[test]
    fn store_no_device() {
        let bus_channel = channel();
        let bus = Bus::new(MachineType::AllRam, &bus_channel.0, Vec::new(), None);
        bus.store(0x0000, 0x00);
    }

    #[test]
    fn store_image() -> Result<()> {
        let descriptions = vec![DeviceDescription {
            address_range: AddressRange::new(5, 7).expect("Must succeed"),
            device_fn: Box::new(|_, image_slice| Box::new(Ram::<3>::new(&image_slice))),
            offset: 5,
        }];

        let bytes = (0..=255).cycle().skip(10).take(100).collect::<Vec<_>>();
        let image = Image::from_bytes(&bytes, None, None, None)?;
        assert_eq!(0x0000, image.load);

        let bus_channel = channel();
        let bus = Bus::new(
            MachineType::AllRam,
            &bus_channel.0,
            descriptions,
            Some(&image),
        );

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
