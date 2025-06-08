use crate::util::make_word;
use crate::{
    Image, MemoryMappedDevice, MemoryView, OsEmulation, Pia, Ram, IRQ, MEMORY_SIZE, NMI, RESET,
};
use anyhow::{bail, Result};

const UNMAPPED_VALUE: u8 = 0xff;

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
        Self::new(vec![DeviceInfo {
            start: 0x0000,
            end: 0xffff,
            device: Box::new(Ram::<MEMORY_SIZE>::default()),
            offset: 0x0000,
        }])
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
        match self.find_device(addr) {
            Some(device) => device.device.load(addr - device.offset),
            None => UNMAPPED_VALUE,
        }
    }

    pub fn store(&self, addr: u16, value: u8) {
        if let Some(device) = self.find_device(addr) {
            device.device.store(addr - device.offset, value)
        }
    }

    fn find_device(&self, addr: u16) -> Option<&DeviceInfo> {
        self.devices
            .iter()
            .find(|&device| addr >= device.start && addr <= device.end)
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::UNMAPPED_VALUE;
    use crate::{DeviceInfo, Image, Memory, Ram};
    use anyhow::Result;

    #[test]
    fn load_no_device() {
        let memory = Memory::new(Vec::new());
        assert_eq!(UNMAPPED_VALUE, memory.load(0x0000));
    }

    #[test]
    fn store_no_device() {
        let memory = Memory::new(Vec::new());
        memory.store(0x0000, 0x00)
    }

    #[test]
    fn store_image() -> Result<()> {
        let devices = vec![DeviceInfo {
            start: 5,
            end: 7,
            device: Box::new(Ram::<3>::default()),
            offset: 5,
        }];
        let memory = Memory::new(devices);
        let bytes = (0..=255).cycle().skip(10).take(100).collect::<Vec<_>>();
        let image = Image::from_bytes(&bytes, None, None, None)?;
        assert_eq!(0x0000, image.load);

        memory.store_image(&image)?;

        for addr in 0..5 {
            assert_eq!(UNMAPPED_VALUE, memory.load(addr));
        }
        assert_eq!(15, memory.load(5));
        assert_eq!(16, memory.load(6));
        assert_eq!(17, memory.load(7));
        for addr in 8..200 {
            assert_eq!(UNMAPPED_VALUE, memory.load(addr));
        }
        Ok(())
    }
}
