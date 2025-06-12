use anyhow::{bail, Error};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct AddressRange(RangeInclusive<u16>);

impl AddressRange {
    #[must_use]
    pub fn overlapping(ranges: &[Self]) -> bool {
        if ranges.is_empty() {
            return false;
        }

        let mut ranges = ranges.to_vec();
        ranges.sort_by_key(AddressRange::start);

        let mut end = ranges.first().expect("Range already checked").end();
        for range in ranges.iter().skip(1) {
            let start = range.start();
            if start <= end {
                return true;
            }
            end = start;
        }

        false
    }

    #[must_use]
    pub fn new(start: u16, end: u16) -> Self {
        assert!(end >= start);
        Self(RangeInclusive::new(start, end))
    }

    #[must_use]
    pub fn start(&self) -> u16 {
        *self.0.start()
    }

    #[must_use]
    pub fn end(&self) -> u16 {
        *self.0.end()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn contains(&self, addr: u16) -> bool {
        addr >= *self.0.start() && addr <= *self.0.end()
    }
}

impl FromStr for AddressRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((prefix, suffix)) => {
                let start = u16::from_str_radix(prefix.trim(), 16)?;
                let end = u16::from_str_radix(suffix.trim(), 16)?;
                Ok(Self::new(start, end))
            }
            None => bail!("invalid address range {s}"),
        }
    }
}

impl Display for AddressRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "${:04X}:${:04X}", self.0.start(), self.0.end())
    }
}

#[cfg(test)]
mod tests {
    use crate::AddressRange;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(AddressRange::new(0x0e00, 0x0e80), 0x0e00, 0x0e80, 0x81, "0e00:0e80")]
    fn basics(
        #[case] expected_result: AddressRange,
        #[case] expected_start: u16,
        #[case] expected_end: u16,
        #[case] expected_len: usize,
        #[case] input: &str,
    ) -> Result<()> {
        let result = input.parse()?;
        assert_eq!(expected_result, result);
        assert_eq!(expected_start, result.start());
        assert_eq!(expected_end, result.end());
        assert_eq!(expected_len, result.len());
        Ok(())
    }

    #[test]
    fn overlapping() {
        assert!(!AddressRange::overlapping(&[
            AddressRange::new(0, 1),
            AddressRange::new(2, 3)
        ]));
        assert!(AddressRange::overlapping(&[
            AddressRange::new(0, 1),
            AddressRange::new(1, 3)
        ]));
        assert!(AddressRange::overlapping(&[
            AddressRange::new(0, 2),
            AddressRange::new(1, 3)
        ]));
    }
}
