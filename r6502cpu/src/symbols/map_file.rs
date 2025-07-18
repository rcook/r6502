use crate::symbols::util::iter_map_file_lines;
use crate::symbols::{Export, Module, Segment};
use anyhow::{Error, Result, anyhow};
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct MapFile {
    pub modules: Vec<Module>,
    pub segments: Vec<Segment>,
    pub exports: Vec<Export>,
}

impl MapFile {
    pub fn load(image_path: &Path) -> Result<Self> {
        let mut file_name = image_path
            .file_stem()
            .ok_or_else(|| anyhow!("could not get file stem"))?
            .to_os_string();
        file_name.push(".map");
        let map_path = image_path
            .parent()
            .ok_or_else(|| anyhow!("could not get parent of path"))?
            .join(file_name);

        if map_path.is_file() {
            let s = read_to_string(map_path)?;
            s.parse()
        } else {
            Ok(Self::default())
        }
    }
}

impl FromStr for MapFile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = iter_map_file_lines(s);
        let modules = Module::fetch_all(&mut lines)?;
        let segments = Segment::fetch_all(&mut lines)?;
        let exports = Export::fetch_all(&mut lines)?;
        Ok(Self {
            modules,
            segments,
            exports,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::MapFile;
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let input = r"Modules list:
-------------
a1basic.o:
    A1BASIC           Offs=000000  Size=001000  Align=00001  Fill=0000
constants.o:
main.o:
    CODE              Offs=000000  Size=000027  Align=00001  Fill=0000
    DATA              Offs=000000  Size=00009C  Align=00001  Fill=0000
    HEADER            Offs=000000  Size=00000A  Align=00001  Fill=0000
    NMI               Offs=000000  Size=000002  Align=00001  Fill=0000
    RESET             Offs=000000  Size=000002  Align=00001  Fill=0000
    IRQ               Offs=000000  Size=000002  Align=00001  Fill=0000
wozmon.o:
    WOZMON            Offs=000000  Size=0000FA  Align=00001  Fill=0000
C:\bin\cc65\lib/none.lib(copydata.o):
    CODE              Offs=000027  Size=00002D  Align=00001  Fill=0000
C:\bin\cc65\lib/none.lib(zeropage.o):
    ZEROPAGE          Offs=000000  Size=00001A  Align=00001  Fill=0000


Segment list:
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

        let map_file = input.parse::<MapFile>()?;
        assert_eq!(6, map_file.modules.len());
        assert_eq!(9, map_file.segments.len());
        assert_eq!(16, map_file.exports.len());
        Ok(())
    }
}
