use crate::util::split_word;
use crate::{Cpu, Opcode, IRQ, NMI, OSHALT, OSIRQ, OSWRCH, RESET};

// Eventually Os will go away!
pub struct Os {
    pub(crate) nmi_addr: Option<u16>,
    pub(crate) reset_addr: Option<u16>,
    pub(crate) irq_addr: Option<u16>,
    pub(crate) return_addr: Option<u16>,
    pub(crate) os_vectors: Vec<u16>,
}

impl Os {
    #[must_use]
    pub fn new(acorn_hack: bool) -> Self {
        if acorn_hack {
            Self {
                nmi_addr: None,
                reset_addr: None,
                irq_addr: Some(OSIRQ),
                return_addr: Some(OSHALT),
                os_vectors: vec![OSHALT, OSWRCH],
            }
        } else {
            Self {
                nmi_addr: None,
                reset_addr: None,
                irq_addr: None,
                return_addr: None,
                os_vectors: vec![],
            }
        }
    }

    pub fn load_into_vm(&self, cpu: &mut Cpu) {
        if let Some(nmi_addr) = self.nmi_addr {
            let (hi, lo) = split_word(nmi_addr);
            cpu.bus.store(NMI, lo);
            cpu.bus.store(NMI.wrapping_add(1), hi);
        }
        if let Some(reset_addr) = self.reset_addr {
            let (hi, lo) = split_word(reset_addr);
            cpu.bus.store(RESET, lo);
            cpu.bus.store(RESET.wrapping_add(1), hi);
        }
        if let Some(irq_addr) = self.irq_addr {
            let (hi, lo) = split_word(irq_addr);
            cpu.bus.store(IRQ, lo);
            cpu.bus.store(IRQ.wrapping_add(1), hi);
        }

        for os_vector in self.os_vectors.iter().copied() {
            cpu.bus.store(os_vector, Opcode::Brk as u8);
            cpu.bus.store(os_vector.wrapping_add(1), Opcode::Nop as u8);
            cpu.bus.store(os_vector.wrapping_add(2), Opcode::Rts as u8);
        }

        if let Some(return_addr) = self.return_addr {
            cpu.push_word(return_addr - 1);
        }
    }

    #[must_use]
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
