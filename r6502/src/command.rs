use crate::AddressRange;
use anyhow::{bail, Error};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum Command {
    FetchMemory(AddressRange),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split_whitespace().collect::<Vec<_>>();
        if parts.len() < 1 {
            bail!("invalid command {s}")
        }

        if parts[0] == "m" || parts[0] == "mem" || parts[0] == "memory" {
            if parts.len() != 2 {
                bail!("invalid \"memory\" command")
            }

            let address_range = parts[1].parse()?;
            return Ok(Self::FetchMemory(address_range));
        }

        bail!("unsupported command {s}");
    }
}

#[cfg(test)]
mod tests {
    use crate::{AddressRange, Command};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(Command::FetchMemory(AddressRange { begin: 0x0e00, end: 0x0eff }), "m e00:eff")]
    fn basics(#[case] expected_result: Command, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_result, input.parse()?);
        Ok(())
    }
}
