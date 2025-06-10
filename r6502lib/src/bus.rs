use crate::util::make_word;
use crate::{
    BusView, DeviceInfo, Image, MachineType, Pia, Ram, Rom, IRQ, MEMORY_SIZE, NMI, PIA_END_ADDR,
    PIA_START_ADDR, RESET,
};
use anyhow::{bail, Result};

const UNMAPPED_VALUE: u8 = 0xff;

// Represents the address bus and attached memory-mapped devices including RAM/ROM/PIA
pub struct Bus {
    devices: Vec<DeviceInfo>,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new(vec![DeviceInfo {
            start: 0x0000,
            end: 0xffff,
            device: Box::new(Ram::<MEMORY_SIZE>::default()),
            offset: 0x0000,
        }])
    }
}

impl Bus {
    #[must_use]
    pub fn configure_for(machine_type: MachineType) -> Self {
        match machine_type {
            MachineType::Acorn => Self::new(vec![
                DeviceInfo {
                    start: 0x0000,
                    end: 0x7fff,
                    device: Box::new(Ram::<0x8000>::default()),
                    offset: 0x0000,
                },
                DeviceInfo {
                    start: 0x8000,
                    end: 0xffff,
                    device: Box::new(Rom::<0x8000>::default()),
                    offset: 0x0000,
                },
            ]),
            MachineType::Apple1 => Self::new(vec![
                DeviceInfo {
                    start: 0x0000,
                    end: PIA_START_ADDR - 1,
                    device: Box::new(Ram::<{ PIA_START_ADDR as usize }>::default()),
                    offset: 0x0000,
                },
                DeviceInfo {
                    start: PIA_START_ADDR,
                    end: PIA_END_ADDR,
                    device: Box::new(Pia::default()),
                    offset: PIA_START_ADDR,
                },
                DeviceInfo {
                    start: PIA_END_ADDR + 1,
                    end: 0xffff,
                    device: Box::new(Ram::<{ 0xffff - PIA_END_ADDR as usize }>::default()),
                    offset: PIA_END_ADDR + 1,
                },
            ]),
            _ => Self::default(),
        }
    }

    #[must_use]
    fn new(mut devices: Vec<DeviceInfo>) -> Self {
        devices.sort_by(|a, b| a.start.cmp(&b.start));
        Self { devices }
    }

    pub fn start(&self) {
        for device in &self.devices {
            device.device.start();
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
    pub fn snapshot(&self, begin: u16, end: u16) -> Vec<u8> {
        let mut result = Vec::with_capacity(end as usize - begin as usize + 1);

        // Incredibly inefficient!
        for addr in begin..=end {
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
        match self.find_device(addr) {
            Some(device) => device.device.load(addr - device.offset),
            None => UNMAPPED_VALUE,
        }
    }

    pub fn store(&self, addr: u16, value: u8) {
        if let Some(device) = self.find_device(addr) {
            device.device.store(addr - device.offset, value);
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
    use crate::bus::UNMAPPED_VALUE;
    use crate::{Bus, DeviceInfo, Image, Ram};
    use anyhow::Result;

    #[test]
    fn load_no_device() {
        let bus = Bus::new(Vec::new());
        assert_eq!(UNMAPPED_VALUE, bus.load(0x0000));
    }

    #[test]
    fn store_no_device() {
        let bus = Bus::new(Vec::new());
        bus.store(0x0000, 0x00);
    }

    #[test]
    fn store_image() -> Result<()> {
        let devices = vec![DeviceInfo {
            start: 5,
            end: 7,
            device: Box::new(Ram::<3>::default()),
            offset: 5,
        }];
        let bus = Bus::new(devices);
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
