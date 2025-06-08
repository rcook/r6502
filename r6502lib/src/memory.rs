use crate::constants::NMI;
use crate::util::make_word;
use crate::{
    Image, MemoryMappedDevice, MemoryView, OsEmulation, Pia, Ram, IRQ, MEMORY_SIZE, RESET,
};
use anyhow::{bail, Result};

// Represents the address bus and attached memory-mapped devices including RAM/ROM/PIA
pub struct Memory {
    devices: Vec<DeviceInfo>,
}

pub struct DeviceInfo {
    start: u16,
    end: u16,
    device: Box<dyn MemoryMappedDevice>,
    offset: u16,
}

impl Default for Memory {
    fn default() -> Self {
        let devices = vec![DeviceInfo {
            start: 0x0000,
            end: 0xffff,
            device: Box::new(Ram::<MEMORY_SIZE>::default()),
            offset: 0x0000,
        }];
        Self::new(devices)
    }
}

impl Memory {
    pub fn emulate(emulation: OsEmulation) -> Self {
        match emulation {
            OsEmulation::Apple1Style => {
                let devices = vec![
                    DeviceInfo {
                        start: 0x0000,
                        end: Pia::START_ADDR - 1,
                        device: Box::new(Ram::<{ Pia::START_ADDR as usize }>::default()),
                        offset: 0x0000,
                    },
                    DeviceInfo {
                        start: Pia::START_ADDR,
                        end: Pia::END_ADDR,
                        device: Box::new(Pia::default()),
                        offset: 0x0000,
                    },
                    DeviceInfo {
                        start: Pia::END_ADDR + 1,
                        end: 0xffff,
                        device: Box::new(Ram::<{ 0xffff - Pia::END_ADDR as usize }>::default()),
                        offset: Pia::END_ADDR + 1,
                    },
                ];
                Self::new(devices)
            }
            _ => Self::default(),
        }
    }
    pub fn new(mut devices: Vec<DeviceInfo>) -> Self {
        devices.sort_by(|a, b| a.start.cmp(&b.start));
        Self { devices }
    }

    pub fn start(&self) {
        for device in &self.devices {
            device.device.start();
        }
    }

    // Don't call this when more than one thread is concurrently accessing memory
    pub fn load_nmi_unsafe(&self) -> u16 {
        let lo = self.load(NMI);
        let hi = self.load(NMI + 1);
        make_word(hi, lo)
    }

    // Don't call this when more than one thread is concurrently accessing memory
    pub fn load_reset_unsafe(&self) -> u16 {
        let lo = self.load(RESET);
        let hi = self.load(RESET + 1);
        make_word(hi, lo)
    }

    // Don't call this when more than one thread is concurrently accessing memory
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
            self.store(i as u16 + load, *value)
        }

        Ok(())
    }

    pub fn snapshot(&self, begin: u16, end: u16) -> Vec<u8> {
        let mut result = Vec::with_capacity(end as usize - begin as usize + 1);

        // Incredibly inefficient!
        for addr in begin..=end {
            result.push(self.load(addr));
        }

        result
    }

    pub fn view(&self) -> MemoryView {
        MemoryView::new(self)
    }

    pub fn load(&self, addr: u16) -> u8 {
        let Some(device) = self.find_device(addr) else {
            todo!()
        };
        device.device.load(addr - device.offset)
    }

    pub fn store(&self, addr: u16, value: u8) {
        let Some(device) = self.find_device(addr) else {
            todo!()
        };
        device.device.store(addr - device.offset, value)
    }

    fn find_device(&self, addr: u16) -> Option<&DeviceInfo> {
        self.devices
            .iter()
            .find(|&device| addr >= device.start && addr <= device.end)
    }
}
