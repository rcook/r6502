use crate::emulator::BusView;
use anyhow::Result;
use r6502core::constants::{IRQ, MEMORY_SIZE, NMI, RESET};
use r6502core::util::make_word;
use r6502core::{AddressRange, MachineTag, NULL_MACHINE_TAG};
use r6502cpu::{DeviceMapping, Ram};
use r6502snapshot::MemoryImage;

const UNMAPPED_VALUE: u8 = 0xff;

// Represents the address bus and attached memory-mapped devices including RAM/ROM/PIA
pub struct Bus {
    machine_tag: MachineTag,
    mappings: Vec<DeviceMapping>,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new(
            NULL_MACHINE_TAG,
            vec![DeviceMapping {
                address_range: AddressRange::new(0x0000, 0xffff).expect("Must succeed"),
                device: Box::new(Ram::new(MEMORY_SIZE, &Vec::new())),
                offset: 0x0000,
            }],
        )
    }
}

impl Bus {
    #[must_use]
    pub fn new(machine_tag: MachineTag, mut mappings: Vec<DeviceMapping>) -> Self {
        assert!(!AddressRange::overlapping(
            &mappings
                .iter()
                .map(|m| m.address_range.clone())
                .collect::<Vec<_>>()
        ));
        mappings.sort_by(|a, b| a.address_range.start().cmp(&b.address_range.start()));
        Self {
            machine_tag,
            mappings,
        }
    }

    pub fn default_with_image(image: &MemoryImage) -> Result<Self> {
        let address_range = AddressRange::new(0x0000, 0xffff)?;
        let memory_slices = [image]
            .iter()
            .map(|image| image.slice(&address_range))
            .collect();
        let device = Box::new(Ram::new(MEMORY_SIZE, &memory_slices));
        Ok(Bus::new(
            image.machine_tag().unwrap_or(NULL_MACHINE_TAG),
            vec![DeviceMapping {
                address_range,
                device,
                offset: 0x0000,
            }],
        ))
    }

    #[must_use]
    pub const fn machine_tag(&self) -> MachineTag {
        self.machine_tag
    }

    pub fn start(&self) {
        for mapping in &self.mappings {
            mapping.device.start();
        }
    }

    #[must_use]
    pub fn stop(&self) -> bool {
        let mut succeeded = true;
        for mapping in &self.mappings {
            if !mapping.device.stop() {
                succeeded = false;
            }
        }
        succeeded
    }

    // Don't call this when more than one thread is concurrently accessing memory
    #[must_use]
    pub fn load_nmi_unsafe(&self) -> u16 {
        let lo = self.load(NMI);
        let hi = self.load(NMI.wrapping_add(1));
        make_word(hi, lo)
    }

    // Don't call this when more than one thread is concurrently accessing memory
    #[must_use]
    pub fn load_reset_unsafe(&self) -> u16 {
        let lo = self.load(RESET);
        let hi = self.load(RESET.wrapping_add(1));
        make_word(hi, lo)
    }

    // Don't call this when more than one thread is concurrently accessing memory
    #[must_use]
    pub fn load_irq_unsafe(&self) -> u16 {
        let lo = self.load(IRQ);
        let hi = self.load(IRQ.wrapping_add(1));
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

    fn find_mapping(&self, addr: u16) -> Option<&DeviceMapping> {
        self.mappings
            .iter()
            .find(|&mapping| mapping.address_range.contains(addr))
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::Bus;
    use crate::emulator::bus::UNMAPPED_VALUE;
    use r6502core::NULL_MACHINE_TAG;

    #[test]
    fn load_no_device() {
        let bus = Bus::new(NULL_MACHINE_TAG, Vec::new());
        assert_eq!(UNMAPPED_VALUE, bus.load(0x0000));
    }

    #[test]
    fn store_no_device() {
        let bus = Bus::new(NULL_MACHINE_TAG, Vec::new());
        bus.store(0x0000, 0x00);
    }
}
