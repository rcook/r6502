use anyhow::{bail, Error};
use std::str::FromStr;

// Represents a closed interval of addresses (includes both addresses)
#[derive(Clone, Debug, PartialEq)]
pub struct AddressRange {
    start: u16,
    end: u16,
}

impl AddressRange {
    #[must_use]
    pub fn new(start: u16, end: u16) -> Self {
        assert!(end >= start);
        Self { start, end }
    }

    #[must_use]
    pub fn start(&self) -> u16 {
        self.start
    }

    #[must_use]
    pub fn end(&self) -> u16 {
        self.end
    }

    #[must_use]
    pub fn contains(&self, addr: u16) -> bool {
        addr >= self.start && addr <= self.end
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

#[cfg(test)]
mod tests {
    use crate::AddressRange;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(AddressRange::new(0x0e00, 0x0e80), "0e00:0e80")]
    fn basics(#[case] expected_result: AddressRange, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_result, input.parse()?);
        Ok(())
    }
}
