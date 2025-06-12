use anyhow::{bail, Error, Result};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct AddressRange(RangeInclusive<u16>);

impl AddressRange {
    pub fn new(start: u16, end: u16) -> Result<Self> {
        if end >= start {
            Ok(Self(RangeInclusive::new(start, end)))
        } else {
            bail!("Invalid address range ${start:02X}:${end:02X}")
        }
    }

    pub fn parse_no_sigils(s: &str) -> Result<Self> {
        match s.split_once(':') {
            Some((prefix, suffix)) => {
                let start = u16::from_str_radix(prefix.trim(), 16)?;
                let end = u16::from_str_radix(suffix.trim(), 16)?;
                Self::new(start, end)
            }
            None => bail!("invalid address range {s}"),
        }
    }

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
    pub const fn start(&self) -> u16 {
        *self.0.start()
    }

    #[must_use]
    pub const fn end(&self) -> u16 {
        *self.0.end()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub const fn contains(&self, addr: u16) -> bool {
        addr >= *self.0.start() && addr <= *self.0.end()
    }
}

impl FromStr for AddressRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((prefix, suffix)) = s.split_once(':') else {
            bail!("invalid address range {s}")
        };

        let Some(s) = prefix.strip_prefix('$') else {
            bail!("invalid address range {s}")
        };

        let start = u16::from_str_radix(s, 16)?;

        let Some(s) = suffix.strip_prefix('$') else {
            bail!("invalid address range {s}")
        };

        let end = u16::from_str_radix(s, 16)?;

        Self::new(start, end)
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
    #[case(AddressRange::new(0x0e00, 0x0e80).expect("Must succeed"), 0x0e00, 0x0e80, 0x81, "0e00:0e80")]
    fn parse_no_sigils(
        #[case] expected_result: AddressRange,
        #[case] expected_start: u16,
        #[case] expected_end: u16,
        #[case] expected_len: usize,
        #[case] input: &str,
    ) -> Result<()> {
        let result = AddressRange::parse_no_sigils(input)?;
        assert_eq!(expected_result, result);
        assert_eq!(expected_start, result.start());
        assert_eq!(expected_end, result.end());
        assert_eq!(expected_len, result.len());
        Ok(())
    }

    #[test]
    fn overlapping() {
        assert!(!AddressRange::overlapping(&[
            AddressRange::new(0, 1).expect("Must succeed"),
            AddressRange::new(2, 3).expect("Must succeed")
        ]));
        assert!(AddressRange::overlapping(&[
            AddressRange::new(0, 1).expect("Must succeed"),
            AddressRange::new(1, 3).expect("Must succeed")
        ]));
        assert!(AddressRange::overlapping(&[
            AddressRange::new(0, 2).expect("Must succeed"),
            AddressRange::new(1, 3).expect("Must succeed")
        ]));
    }
}
