use crate::util::split_word;
use crate::{constants::NMI, Cpu, Opcode, OsEmulation, IRQ, OSHALT, OSWRCH, RESET};
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

    pub fn load_into_vm(&self, cpu: &mut Cpu) {
        if let Some(nmi_addr) = self.nmi_addr {
            let (hi, lo) = split_word(nmi_addr);
            cpu.memory.store(NMI, lo);
            cpu.memory.store(NMI.wrapping_add(1), hi)
        }
        if let Some(reset_addr) = self.reset_addr {
            let (hi, lo) = split_word(reset_addr);
            cpu.memory.store(RESET, lo);
            cpu.memory.store(RESET.wrapping_add(1), hi)
        }
        if let Some(irq_addr) = self.irq_addr {
            let (hi, lo) = split_word(irq_addr);
            cpu.memory.store(IRQ, lo);
            cpu.memory.store(IRQ.wrapping_add(1), hi)
        }

        for os_vector in self.os_vectors.iter().cloned() {
            cpu.memory.store(os_vector, Opcode::Brk as u8);
            cpu.memory
                .store(os_vector.wrapping_add(1), Opcode::Nop as u8);
            cpu.memory
                .store(os_vector.wrapping_add(2), Opcode::Rts as u8);
        }

        if let Some(return_addr) = self.return_addr {
            cpu.push_word(return_addr - 1);
        }
    }

    pub fn is_os_vector(&self, cpu: &Cpu) -> Option<u16> {
        match self.irq_addr {
            Some(irq) if cpu.reg.pc == irq => {
                let addr = cpu.peek_back_word(1).wrapping_sub(2);
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
