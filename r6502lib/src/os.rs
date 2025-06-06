use crate::{constants::NMI, Opcode, OsEmulation, Vm, IRQ, OSHALT, OSWRCH, RESET};
use anyhow::Result;
use derive_builder::Builder;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Os {
    #[builder(default, setter(strip_option))]
    nmi_addr: Option<u16>,

    #[builder(default, setter(strip_option))]
    reset_addr: Option<u16>,

    #[builder(default, setter(strip_option))]
    irq_addr: Option<u16>,

    #[builder(default, setter(strip_option))]
    return_addr: Option<u16>,

    #[builder(default)]
    os_vectors: Vec<u16>,
}

impl Os {
    pub fn emulate(emulation: OsEmulation) -> Result<Self> {
        Ok(match emulation {
            OsEmulation::None => OsBuilder::default().build()?,
            OsEmulation::Sim6502 => todo!(),
            OsEmulation::AcornStyle => OsBuilder::default()
                .irq_addr(0x8000)
                .return_addr(OSHALT)
                .os_vectors(vec![OSHALT, OSWRCH])
                .build()?,
            OsEmulation::Apple1Style => OsBuilder::default().build()?,
        })
    }

    pub fn load_into_vm(&self, vm: &mut Vm) {
        if let Some(nmi_addr) = self.nmi_addr {
            vm.s.memory.store_word(NMI, nmi_addr);
        }
        if let Some(reset_addr) = self.reset_addr {
            vm.s.memory.store_word(RESET, reset_addr);
        }
        if let Some(irq_addr) = self.irq_addr {
            vm.s.memory.store_word(IRQ, irq_addr);
        }

        for os_vector in self.os_vectors.iter().cloned() {
            vm.s.memory[os_vector] = Opcode::Brk as u8;
            vm.s.memory[os_vector + 1] = Opcode::Nop as u8;
            vm.s.memory[os_vector + 2] = Opcode::Rts as u8;
        }

        if let Some(return_addr) = self.return_addr {
            vm.s.push_word(return_addr - 1);
        }
    }

    pub fn is_os_vector(&self, vm: &Vm) -> Option<u16> {
        match self.irq_addr {
            Some(irq) if vm.s.reg.pc == irq => {
                let addr = vm.s.peek_back_word(1).wrapping_sub(2);
                if self.os_vectors.contains(&addr) {
                    Some(addr)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
