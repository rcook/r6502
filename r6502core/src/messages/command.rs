use anyhow::{Error, bail};
use r6502lib::AddressRange;
use std::str::FromStr;

const HELP: &str = "?/h/help: Show help message\n\
    m/mem/memory <START>(:<END>): Dump block of memory\n\
    pc <ADDRESS>: Set program counter\n\
    go <ADDRESS>: Set program counter and start program\n";

#[derive(Debug, PartialEq)]
pub enum Command {
    Help(&'static str),
    FetchMemory(AddressRange),
    SetPc(u16),
    Go(u16),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.is_empty() {
            bail!("invalid command {s}")
        }

        // Help
        if parts[0] == "?" || parts[0] == "h" || parts[0] == "help" {
            return Ok(Self::Help(HELP));
        }

        // Fetch snapshot of memory
        if parts[0] == "m" || parts[0] == "mem" || parts[0] == "memory" {
            if parts.len() != 2 {
                bail!("invalid \"memory\" command")
            }

            let address_range = AddressRange::parse_no_sigils(parts[1])?;
            return Ok(Self::FetchMemory(address_range));
        }

        // Set program counter
        if parts[0] == "pc" {
            if parts.len() != 2 {
                bail!("invalid \"pc\" command")
            }

            let addr = u16::from_str_radix(parts[1].trim(), 16)?;
            return Ok(Self::SetPc(addr));
        }

        // Set program counter, clear B and restart
        if parts[0] == "go" {
            if parts.len() != 2 {
                bail!("invalid \"go\" command")
            }

            let addr = u16::from_str_radix(parts[1].trim(), 16)?;
            return Ok(Self::Go(addr));
        }

        bail!("unsupported command {s}");
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::Command;
    use anyhow::Result;
    use r6502lib::AddressRange;
    use rstest::rstest;

    #[rstest]
    #[case(Command::FetchMemory(AddressRange::new(0x0e00, 0x0eff).expect("Must succeed")), "m e00:eff")]
    fn basics(#[case] expected_result: Command, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_result, input.parse()?);
        Ok(())
    }
}
