use crate::emulator::{BusDevice, ImageSlice};
use std::sync::atomic::{AtomicU8, Ordering};

pub struct Ram {
    bytes: Vec<AtomicU8>,
}

impl Ram {
    #[must_use]
    pub fn new(size: usize, image_slices: &Vec<ImageSlice>) -> Self {
        let mut bytes = Vec::with_capacity(size);
        bytes.resize_with(size, || AtomicU8::new(0));
        for image_slice in image_slices {
            let load = image_slice.load as usize;
            for (i, value) in image_slice.bytes.iter().enumerate() {
                bytes[load + i].store(*value, Ordering::SeqCst);
            }
        }
        Self { bytes }
    }
}

impl BusDevice for Ram {
    fn start(&self) {}

    fn load(&self, addr: u16) -> u8 {
        self.bytes[addr as usize].load(Ordering::SeqCst)
    }

    fn store(&self, addr: u16, value: u8) {
        self.bytes[addr as usize].store(value, Ordering::SeqCst);
    }

    fn join(&self) {}
}
