use crate::util::make_word;
use crate::{SymbolInfo, MAGIC_NUMBER};
use anyhow::{anyhow, bail, Error, Result};
use std::fs::File;
use std::io::{Cursor, ErrorKind, Read, Seek};
use std::path::Path;
use std::str::FromStr;

pub(crate) struct Image {
    #[allow(unused)]
    pub(crate) origin: u16,

    #[allow(unused)]
    pub(crate) start: u16,

    #[allow(unused)]
    pub(crate) values: Vec<u8>,

    #[allow(unused)]
    pub(crate) symbols: Vec<SymbolInfo>,
}

#[allow(unused)]
impl Image {
    pub(crate) fn load(
        path: &Path,
        default_origin: Option<u16>,
        default_start: Option<u16>,
    ) -> Result<Self> {
        let symbols = Self::load_symbols(path)?;
        Self::read(File::open(path)?, default_origin, default_start, symbols)
    }

    pub(crate) fn from_bytes(
        bytes: &[u8],
        default_origin: Option<u16>,
        default_start: Option<u16>,
    ) -> Result<Self> {
        Self::read(
            Cursor::new(bytes),
            default_origin,
            default_start,
            Vec::new(),
        )
    }

    fn read<R: Read + Seek>(
        mut reader: R,
        default_origin: Option<u16>,
        default_start: Option<u16>,
        symbols: Vec<SymbolInfo>,
    ) -> Result<Self> {
        let (origin, start) = Self::read_header(&mut reader, default_origin, default_start)?;
        let mut values = Vec::new();
        reader.read_to_end(&mut values)?;
        Ok(Self {
            origin,
            start,
            values,
            symbols,
        })
    }

    fn read_header<R: Read + Seek>(
        reader: &mut R,
        default_origin: Option<u16>,
        default_start: Option<u16>,
    ) -> Result<(u16, u16)> {
        let mut header = [0x00u8; 6];
        match reader.read_exact(&mut header) {
            Ok(()) => {
                let magic_number = make_word(header[1], header[0]);
                if magic_number == MAGIC_NUMBER {
                    let origin = make_word(header[3], header[2]);
                    let start = make_word(header[5], header[4]);
                    Ok((origin, start))
                } else {
                    reader.rewind()?;
                    Ok((
                        default_origin.unwrap_or(0x0000),
                        default_start.unwrap_or(0x0000),
                    ))
                }
            }
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                reader.rewind()?;
                Ok((
                    default_origin.unwrap_or(0x0000),
                    default_start.unwrap_or(0x0000),
                ))
            }
            Err(e) => bail!(e),
        }
    }

    fn load_symbols(path: &Path) -> Result<Vec<SymbolInfo>> {
        let mut file_name = path
            .file_name()
            .ok_or_else(|| anyhow!("could not get file name"))?
            .to_os_string();
        file_name.push(".json");
        let symbol_path = path
            .parent()
            .ok_or_else(|| anyhow!("could not get parent of path"))?
            .join(file_name);

        Ok(if symbol_path.is_file() {
            let file = File::open(symbol_path)?;
            serde_json::from_reader(file)?
        } else {
            Vec::new()
        })
    }
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
                start: 0x0000,
                values,
                symbols: Vec::new(),
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

        let start = origin;
        Ok(Self {
            origin,
            start,
            values,
            symbols: Vec::new(),
        })
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
