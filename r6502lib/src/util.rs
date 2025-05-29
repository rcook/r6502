pub(crate) fn make_word(hi: u8, lo: u8) -> u16 {
    ((hi as u16) << 8) + lo as u16
}

pub(crate) fn split_word(value: u16) -> (u8, u8) {
    let hi = (value >> 8) as u8;
    let lo = value as u8;
    (hi, lo)
}

#[cfg(test)]
mod tests {
    use crate::util::make_word;
    use rstest::rstest;

    #[rstest]
    #[case(0x1234, 0x12, 0x34)]
    fn basics(#[case] expected: u16, #[case] hi: u8, #[case] lo: u8) {
        assert_eq!(expected, make_word(hi, lo))
    }
}
