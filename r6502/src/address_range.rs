use anyhow::{bail, Error};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) struct AddressRange {
    pub(crate) begin: u16,
    pub(crate) end: u16,
}

impl FromStr for AddressRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((prefix, suffix)) => {
                let begin = u16::from_str_radix(prefix.trim(), 16)?;
                let end = u16::from_str_radix(suffix.trim(), 16)?;
                Ok(Self { begin, end })
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
    #[case(AddressRange { begin: 0x0e00, end: 0x0e80 }, "0e00:0e80")]
    fn basics(#[case] expected_result: AddressRange, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_result, input.parse()?);
        Ok(())
    }
}
