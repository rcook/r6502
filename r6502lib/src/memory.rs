use std::ops::{Index, IndexMut};

pub(crate) struct Memory([u8; 0x10000]);

impl Memory {
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self([0x00; 0x10000])
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
