use crate::util::{make_word, split_word};
use crate::Image;
use std::ops::{Index, IndexMut};

pub struct Memory([u8; 0x10000]);

impl Default for Memory {
    fn default() -> Self {
        Self([0x00; 0x10000])
    }
}

impl Memory {
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self, image: &Image) {
        let origin = image.origin as usize;
        let limit = origin + image.values.len();
        self.0[origin..limit].copy_from_slice(&image.values);
    }

    pub fn snapshot(&self, begin: usize, end: usize) -> Vec<u8> {
        self.0[begin..end].to_vec()
    }

    pub(crate) fn fetch_word(&self, addr: u16) -> u16 {
        let lo = self[addr];
        let hi = self[addr + 1];
        make_word(hi, lo)
    }

    #[allow(unused)]
    pub(crate) fn store_word(&mut self, addr: u16, value: u16) {
        let (hi, lo) = split_word(value);
        self[addr] = lo;
        self[addr + 1] = hi;
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
