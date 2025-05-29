use anyhow::{bail, Error, Result};
use std::str::FromStr;

pub(crate) struct Image {
    #[allow(unused)]
    pub(crate) origin: u16,

    #[allow(unused)]
    pub(crate) values: Vec<u8>,
}

impl FromStr for Image {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn read_line(values: &mut Vec<u8>, line: &str) -> Result<(u16, u16)> {
            let mut i = line.split_whitespace();

            let Some(s) = i.next() else {
                bail!("input line is empty");
            };

            if s.len() != 4 {
                bail!("invalid address {s}")
            }

            let Ok(addr) = u16::from_str_radix(s, 16) else {
                bail!("invalid address {s}")
            };

            let mut count = 0;
            for s in i {
                if s.len() != 2 {
                    break;
                }

                let Ok(value) = u8::from_str_radix(s, 16) else {
                    break;
                };

                values.push(value);
                count += 1;
            }

            Ok((addr, count))
        }

        let mut values = Vec::new();
        let mut i = s.lines();
        let Some(line) = i.next() else {
            return Ok(Self {
                origin: 0x0000,
                values,
            });
        };

        let (mut pc, count) = read_line(&mut values, line)?;

        let origin = pc;

        let (temp_pc, overflowed) = pc.overflowing_add(count);
        if overflowed {
            bail!("too much data")
        }

        pc = temp_pc;

        for line in i {
            let (addr, count) = read_line(&mut values, line)?;
            if addr != pc {
                bail!("invalid assembly listing")
            }

            let (temp_pc, overflowed) = pc.overflowing_add(count);
            if overflowed {
                bail!("too much data")
            }

            pc = temp_pc;
        }

        Ok(Self { origin, values })
    }
}

#[cfg(test)]
mod tests {
    use crate::Image;
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let input = r#" 0E00  A2 00     LDX  #$00
 0E02  BD 0E 0E  LDA  $0E0E, X
 0E05  F0 06     BEQ  $0E0D
 0E07  20 EE FF  JSR  $FFEE
 0E0A  E8        INX
 0E0B  D0 F5     BNE  $0E02
 0E0D  60        RTS
 0E0E  48 45 4C 4C 4F 2C 20 57 4F 52 4C 44 21 00        |HELLO, WORLD!.  |
"#;
        let image = input.parse::<Image>()?;
        assert_eq!(0x0e00, image.origin);
        assert_eq!(28, image.values.len());
        Ok(())
    }
}
