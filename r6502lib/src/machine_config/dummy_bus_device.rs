use crate::emulator::BusDevice;

pub struct DummyBusDevice;

impl BusDevice for DummyBusDevice {
    fn load(&self, _addr: u16) -> u8 {
        0x00
    }

    fn store(&self, _addr: u16, _value: u8) {}
}
