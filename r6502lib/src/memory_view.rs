use crate::Memory;

pub struct MemoryView<'a> {
    memory: &'a Memory,
}

impl<'a> MemoryView<'a> {
    pub fn new(memory: &'a Memory) -> Self {
        Self { memory }
    }

    pub fn load(&self, addr: u16) -> u8 {
        self.memory.load(addr)
    }

    pub fn store(&self, addr: u16, value: u8) {
        self.memory.store(addr, value)
    }
}
