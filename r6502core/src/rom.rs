use crate::BusDevice;
use r6502snapshot::MemorySlice;

pub struct Rom {
    bytes: Vec<u8>,
}

impl Rom {
    #[must_use]
    pub fn new(size: usize, memory_slices: &Vec<MemorySlice>) -> Self {
        let mut bytes = vec![0x00; size];
        for memory_slice in memory_slices {
            let load = memory_slice.load as usize;
            bytes[load..load + memory_slice.bytes.len()].copy_from_slice(memory_slice.bytes);
        }
        Self { bytes }
    }
}

impl BusDevice for Rom {
    fn load(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    fn store(&self, _addr: u16, _value: u8) {}
}
