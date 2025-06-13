use crate::{BusDevice, ImageSlice};

pub struct Rom {
    bytes: Vec<u8>,
}

impl Rom {
    #[must_use]
    pub fn new(size: usize, image_slices: &Vec<ImageSlice>) -> Self {
        let mut bytes = vec![0x00; size];
        for image_slice in image_slices {
            let load = image_slice.load as usize;
            bytes[load..load + image_slice.bytes.len()].copy_from_slice(image_slice.bytes);
        }
        Self { bytes }
    }
}

impl BusDevice for Rom {
    fn start(&self) {}

    fn load(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    fn store(&self, _addr: u16, _value: u8) {}

    fn join(&self) {}
}
