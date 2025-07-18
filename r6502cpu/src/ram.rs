use crate::{BusDevice, MemorySlice};
use std::sync::atomic::{AtomicU8, Ordering};

pub struct Ram {
    bytes: Vec<AtomicU8>,
}

impl Ram {
    #[must_use]
    pub fn new(size: usize, memory_slices: &Vec<MemorySlice>) -> Self {
        let mut bytes = Vec::with_capacity(size);
        bytes.resize_with(size, || AtomicU8::new(0));
        for memory_slice in memory_slices {
            let load = memory_slice.load as usize;
            for (i, value) in memory_slice.bytes.iter().enumerate() {
                bytes[load + i].store(*value, Ordering::SeqCst);
            }
        }
        Self { bytes }
    }
}

impl BusDevice for Ram {
    fn load(&self, addr: u16) -> u8 {
        self.bytes[addr as usize].load(Ordering::SeqCst)
    }

    fn store(&self, addr: u16, value: u8) {
        self.bytes[addr as usize].store(value, Ordering::SeqCst);
    }
}
