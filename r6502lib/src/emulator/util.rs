use crate::emulator::{Cpu, IRQ};

#[must_use]
pub const fn make_word(hi: u8, lo: u8) -> u16 {
    ((hi as u16) << 8) + lo as u16
}

#[must_use]
pub const fn split_word(value: u16) -> (u8, u8) {
    let hi = (value >> 8) as u8;
    let lo = value as u8;
    (hi, lo)
}

pub(crate) const fn crosses_page_boundary(addr: u16) -> bool {
    (addr & 0x00ff) == 0x00ff
}

// https://stackoverflow.com/questions/46262435/indirect-y-indexed-addressing-mode-in-mos-6502
pub(crate) fn compute_effective_addr_indirect_indexed_y(cpu: &mut Cpu, addr: u8) -> u16 {
    let (lo, carry) = cpu.bus.load(addr as u16).overflowing_add(cpu.reg.y);
    let next_addr = addr.wrapping_add(1);
    let hi = cpu
        .bus
        .load(next_addr as u16)
        .wrapping_add(if carry { 1 } else { 0 });
    make_word(hi, lo)
}

pub(crate) fn compute_effective_addr_indexed_indirect_x(cpu: &mut Cpu, addr: u8) -> u16 {
    let addr_with_index = addr.wrapping_add(cpu.reg.x);
    let lo = cpu.bus.load(addr_with_index as u16);
    let hi = cpu.bus.load(addr_with_index.wrapping_add(1) as u16);
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

#[cfg(test)]
mod tests {
    use crate::emulator::util::{crosses_page_boundary, make_word};
    use rstest::rstest;

    #[rstest]
    #[case(0x1234, 0x12, 0x34)]
    fn make_word_basics(#[case] expected: u16, #[case] hi: u8, #[case] lo: u8) {
        assert_eq!(expected, make_word(hi, lo));
    }

    #[rstest]
    #[case(false, 0x0000)]
    #[case(true, 0x00ff)]
    #[case(false, 0x0100)]
    #[case(true, 0x01ff)]
    fn crosses_page_boundary_basics(#[case] expected_result: bool, #[case] input: u16) {
        assert_eq!(expected_result, crosses_page_boundary(input));
    }
}
