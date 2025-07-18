use anyhow::{Error, Result, bail};
use std::{iter::Peekable, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct Segment {
    pub name: String,
    pub start: u32,
    pub end: u32,
    pub size: u32,
    pub align: u32,
}

impl Segment {
    pub fn fetch_all<'a>(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Result<Vec<Self>> {
        if lines.peek() != Some(&"Segment list:") {
            bail!("invalid segment list");
        }

        _ = lines.next();

        let Some(s) = lines.next() else {
            bail!("invalid segment list");
        };

        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts != vec!["Name", "Start", "End", "Size", "Align"] {
            bail!("invalid segment list");
        }

        let mut segments = Vec::new();
        loop {
            let line = lines.peek();
            if matches!(line, None | Some(&"Exports list by name:")) {
                break;
            }

            let s = lines.next().expect("Peeked previously");

            let Ok(segment) = s.parse() else {
                bail!("invalid segment list");
            };

            segments.push(segment);
        }

        Ok(segments)
    }
}

impl FromStr for Segment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 5 {
            bail!("invalid segment \"{s}\"")
        }

        let name = String::from(parts[0]);
        let start = u32::from_str_radix(parts[1], 16)?;
        let end = u32::from_str_radix(parts[2], 16)?;
        let size = u32::from_str_radix(parts[3], 16)?;
        let align = u32::from_str_radix(parts[4], 16)?;

        Ok(Self {
            name,
            start,
            end,
            size,
            align,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::Segment;
    use crate::symbols::util::iter_map_file_lines;
    use anyhow::Result;

    #[test]
    fn fetch_all() -> Result<()> {
        let input = r"Segment list:
-------------
Name                   Start     End    Size  Align
----------------------------------------------------
HEADER                000000  000009  00000A  00001
ZEROPAGE              000000  000019  00001A  00001
DATA                  005000  00509B  00009C  00001
CODE                  00D016  00D069  000054  00001
A1BASIC               00E000  00EFFF  001000  00001
WOZMON                00FF00  00FFF9  0000FA  00001
NMI                   00FFFA  00FFFB  000002  00001
RESET                 00FFFC  00FFFD  000002  00001
IRQ                   00FFFE  00FFFF  000002  00001


Exports list by name:
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

        let segments = Segment::fetch_all(&mut lines)?;
        assert_eq!(9, segments.len());

        assert_eq!(Some(&"Exports list by name:"), lines.peek());
        Ok(())
    }

    #[test]
    fn parse() -> Result<()> {
        let segment = "HEADER                000000  000009  00000A  00001".parse::<Segment>()?;
        assert_eq!("HEADER", segment.name);
        assert_eq!(0x00_0000, segment.start);
        assert_eq!(0x00_0009, segment.end);
        assert_eq!(0x00_000a, segment.size);
        assert_eq!(0x0_0001, segment.align);
        Ok(())
    }
}
