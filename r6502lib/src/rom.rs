use crate::MemoryMappedDevice;

pub struct Rom<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> Default for Rom<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Rom<N> {
    #[must_use]
    pub fn new() -> Self {
        Self { bytes: [0x00; N] }
    }
}

impl<const N: usize> MemoryMappedDevice for Rom<N> {
    fn start(&self) {}

    fn load(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    fn store(&self, _addr: u16, _value: u8) {}
}
