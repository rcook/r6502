use anyhow::{bail, Error};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ModuleSegment {
    pub name: String,
    pub offset: u32,
    pub size: u32,
    pub align: u32,
    pub fill: u16,
}

impl FromStr for ModuleSegment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 5 {
            bail!("invalid segment \"{s}\"")
        }

        let name = String::from(parts[0]);

        let Some(temp) = parts[1].strip_prefix("Offs=") else {
            bail!("invalid segment \"{s}\"")
        };

        let offset = u32::from_str_radix(temp, 16)?;

        let Some(temp) = parts[2].strip_prefix("Size=") else {
            bail!("invalid segment \"{s}\"")
        };

        let size = u32::from_str_radix(temp, 16)?;

        let Some(temp) = parts[3].strip_prefix("Align=") else {
            bail!("invalid segment \"{s}\"")
        };

        let align = u32::from_str_radix(temp, 16)?;

        let Some(temp) = parts[4].strip_prefix("Fill=") else {
            bail!("invalid segment \"{s}\"")
        };

        let fill = u16::from_str_radix(temp, 16)?;

        Ok(Self {
            name,
            offset,
            size,
            align,
            fill,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::ModuleSegment;
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let segment = "    DATA              Offs=000000  Size=00009C  Align=00001  Fill=0000"
            .parse::<ModuleSegment>()?;
        assert_eq!("DATA", segment.name);
        assert_eq!(0x00_0000, segment.offset);
        assert_eq!(0x00_009c, segment.size);
        assert_eq!(0x0_0001, segment.align);
        assert_eq!(0x0000, segment.fill);
        Ok(())
    }
}
