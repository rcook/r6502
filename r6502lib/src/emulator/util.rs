use crate::emulator::Cpu;
use r6502core::util::make_word;
use r6502cpu::constants::IRQ;

// https://stackoverflow.com/questions/46262435/indirect-y-indexed-addressing-mode-in-mos-6502
pub fn compute_effective_addr_indirect_indexed_y(cpu: &mut Cpu, addr: u8) -> u16 {
    let (lo, carry) = cpu.bus.load(u16::from(addr)).overflowing_add(cpu.reg.y);
    let next_addr = addr.wrapping_add(1);
    let hi = cpu
        .bus
        .load(u16::from(next_addr))
        .wrapping_add(u8::from(carry));
    make_word(hi, lo)
}

pub fn compute_effective_addr_indexed_indirect_x(cpu: &mut Cpu, addr: u8) -> u16 {
    let addr_with_index = addr.wrapping_add(cpu.reg.x);
    let lo = cpu.bus.load(u16::from(addr_with_index));
    let hi = cpu.bus.load(u16::from(addr_with_index.wrapping_add(1)));
    make_word(hi, lo)
}

#[must_use]
pub fn get_brk_addr(cpu: &Cpu) -> Option<u16> {
    let lo = cpu.bus.load(IRQ);
    let hi = cpu.bus.load(IRQ.wrapping_add(1));
    let current_irq_addr = make_word(hi, lo);

    if cpu.reg.pc != current_irq_addr {
        return None;
    }

    let addr = cpu.peek_back_word(1).wrapping_sub(2);
    Some(addr)
}
