use crate::emulator::{Cpu, IRQ};
use crate::num::Truncate;
use anyhow::Result;
use chrono::Utc;
use std::env::current_dir;
use std::path::PathBuf;

pub fn make_unique_snapshot_path() -> Result<PathBuf> {
    let now = Utc::now();
    let file_name = format!(
        "r6502-snapshot-{timestamp}.r6502",
        timestamp = now.format("%Y%m%d%H%M%S")
    );

    Ok(current_dir()?.join(file_name))
}

#[must_use]
pub const fn make_word(hi: u8, lo: u8) -> u16 {
    ((hi as u16) << 8) + lo as u16
}

#[must_use]
pub fn split_word(value: u16) -> (u8, u8) {
    let hi = (value >> 8) as u8;
    let lo = u8::truncate(value);
    (hi, lo)
}

#[must_use]
pub const fn crosses_page_boundary(addr: u16) -> bool {
    (addr & 0x00ff) == 0x00ff
}

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
