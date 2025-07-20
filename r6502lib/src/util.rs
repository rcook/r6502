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

#[cfg(test)]
mod tests {
    use crate::util::{crosses_page_boundary, make_word};
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
