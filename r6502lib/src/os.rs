use crate::{Memory, Opcode, Vm, IRQ, OSHALT, OSWRCH};
use derive_builder::Builder;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Os {
    #[builder(default = 0x8000)]
    os_addr: u16,

    #[builder(default = "self.default_os_vectors()")]
    os_vectors: Vec<u16>,
}

impl Os {
    pub fn initialize(&self, memory: &mut Memory) {
        memory.store_word(IRQ, self.os_addr);

        for os_vector in self.os_vectors.iter().cloned() {
            memory[os_vector] = Opcode::Brk as u8;
            memory[os_vector + 1] = Opcode::Nop as u8;
            memory[os_vector + 2] = Opcode::Rts as u8;
        }
    }

    pub fn is_os_vector(&self, vm: &Vm) -> Option<u16> {
        if vm.s.reg.pc == self.os_addr {
            let addr = vm.s.peek_back_word(1).wrapping_sub(2);
            if self.os_vectors.contains(&addr) {
                Some(addr)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl OsBuilder {
    fn default_os_vectors(&self) -> Vec<u16> {
        vec![OSHALT, OSWRCH]
    }
}
