use crate::symbols::{AddressSize, ExportKind};
use anyhow::{bail, Error, Result};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::iter::Peekable;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Export {
    pub name: String,
    pub value: u32,
    pub referenced: bool,
    pub kind: ExportKind,
    pub address_size: AddressSize,
}

impl Export {
    pub fn fetch_all<'a>(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Result<Vec<Self>> {
        if lines.peek() != Some(&"Exports list by name:") {
            bail!("Invalid export list")
        }

        _ = lines.next();

        let mut exports = Vec::new();
        loop {
            let line = lines.peek();
            if matches!(line, None | Some(&"Exports list by value:")) {
                break;
            }

            let s = lines.next().expect("Peeked previously");
            let parts = s.split_whitespace().collect::<Vec<_>>();
            let len = parts.len();
            if len != 3 && len != 6 {
                bail!("Invalid export list");
            }

            exports.push(Self::parse_export(parts[0], parts[1], parts[2])?);
            if len == 6 {
                exports.push(Self::parse_export(parts[3], parts[4], parts[5])?);
            }
        }

        loop {
            let line = lines.peek();
            if matches!(line, None | Some(&"Imports list:")) {
                break;
            }

            _ = lines.next();
        }

        exports.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(exports)
    }

    fn parse_export(s0: &str, s1: &str, s2: &str) -> Result<Self> {
        let name = String::from(s0);
        let value = u32::from_str_radix(s1, 16)?;

        let (referenced, s3) = match s2.strip_prefix('R') {
            Some(s) => (true, s),
            None => (false, s2),
        };

        if s3.len() != 2 {
            bail!("Invalid export flags {s2}");
        }

        let mut i = s3.chars();

        let kind = match i.next() {
            Some('L') => ExportKind::Label,
            Some('E') => ExportKind::Constant,
            _ => bail!("Invalid export flags {s2}"),
        };

        let address_size = match i.next() {
            Some('Z') => AddressSize::ZeroPage,
            Some('A') => AddressSize::Absolute,
            Some('F') => AddressSize::Far,
            Some('L') => AddressSize::Long,
            _ => bail!("Invalid export flags {s2}"),
        };

        Ok(Self {
            name,
            value,
            referenced,
            kind,
            address_size,
        })
    }
}

impl Display for Export {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{name} = ", name = self.name)?;
        match self.address_size {
            AddressSize::ZeroPage => write!(f, "${value:02X} ", value = self.value)?,
            AddressSize::Absolute => write!(f, "${value:04X} ", value = self.value)?,
            AddressSize::Far => write!(f, "${value:06X} ", value = self.value)?,
            AddressSize::Long => write!(f, "${value:08X} ", value = self.value)?,
        }
        match self.kind {
            ExportKind::Label => write!(f, "(label)")?,
            ExportKind::Constant => write!(f, "(constant)")?,
        }
        if !self.referenced {
            write!(f, " (unreferenced)")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::util::iter_map_file_lines;
    use crate::symbols::Export;
    use anyhow::Result;

    #[test]
    fn fetch_all() -> Result<()> {
        let input = r"Exports list by name:
---------------------
DSP                       00D012 REA    DSPCR                     00D013 REA    
ECHO                      00FFEF RLA    GETLINE                   00FF1F RLA    
HALT                      00D014 REA    KBDCR                     00D011 REA    
PRBYTE                    00FFDC RLA    PRHEX                     00FFE5 RLA    
WOZMON                    00FF00 RLA    __DATA_LOAD__             009000 RLA    
__DATA_RUN__              005000 RLA    __DATA_SIZE__             00009C REA    
copydata                  00D03D RLA    ptr1                      000008 RLZ    
ptr2                      00000A RLZ    tmp1                      000010 RLZ    



Exports list by value:
----------------------
ptr1                      000008 RLZ    ptr2                      00000A RLZ    
tmp1                      000010 RLZ    __DATA_SIZE__             00009C REA    
__DATA_RUN__              005000 RLA    __DATA_LOAD__             009000 RLA    
KBDCR                     00D011 REA    DSP                       00D012 REA    
DSPCR                     00D013 REA    HALT                      00D014 REA    
copydata                  00D03D RLA    WOZMON                    00FF00 RLA    
GETLINE                   00FF1F RLA    PRBYTE                    00FFDC RLA    
PRHEX                     00FFE5 RLA    ECHO                      00FFEF RLA    



Imports list:
-------------
DSP (wozmon.o):
    main.o                    main.s:11
DSPCR (wozmon.o):
    main.o                    main.s:12
ECHO (wozmon.o):
    main.o                    main.s:15
GETLINE (wozmon.o):
    main.o                    main.s:16
HALT (constants.o):
    main.o                    main.s:13
KBDCR (wozmon.o):
    main.o                    main.s:14
PRBYTE (wozmon.o):
    main.o                    main.s:17
PRHEX (wozmon.o):
    main.o                    main.s:18
WOZMON (wozmon.o):
    main.o                    main.s:19
__DATA_LOAD__ ([linker generated]):
    copydata.o                common/copydata.s:8
    main.o                    main.s:3
__DATA_RUN__ ([linker generated]):
    copydata.o                common/copydata.s:8
__DATA_SIZE__ ([linker generated]):
    copydata.o                common/copydata.s:8
copydata (copydata.o):
    main.o                    main.s:62
ptr1 (zeropage.o):
    copydata.o                common/copydata.s:9
ptr2 (zeropage.o):
    copydata.o                common/copydata.s:9
tmp1 (zeropage.o):
    copydata.o                common/copydata.s:9

";

        let mut lines = iter_map_file_lines(input);

        let exports = Export::fetch_all(&mut lines)?;
        assert_eq!(16, exports.len());

        assert_eq!(Some(&"Imports list:"), lines.peek());
        Ok(())
    }
}
