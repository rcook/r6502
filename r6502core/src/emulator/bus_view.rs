use crate::emulator::Bus;
use r6502lib::MachineTag;

#[derive(Clone)]
pub struct BusView<'a> {
    bus: &'a Bus,
}

impl<'a> BusView<'a> {
    #[must_use]
    pub const fn new(bus: &'a Bus) -> Self {
        Self { bus }
    }

    #[must_use]
    pub const fn machine_tag(&self) -> MachineTag {
        self.bus.machine_tag()
    }

    #[must_use]
    pub fn load(&self, addr: u16) -> u8 {
        self.bus.load(addr)
    }

    pub fn store(&self, addr: u16, value: u8) {
        self.bus.store(addr, value);
    }
}
