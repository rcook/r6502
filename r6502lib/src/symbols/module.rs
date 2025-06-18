use crate::symbols::{ModuleName, ModuleSegment};
use anyhow::{bail, Result};
use std::iter::Peekable;

#[derive(Debug)]
pub struct Module {
    pub name: ModuleName,
    pub segments: Vec<ModuleSegment>,
}

impl Module {
    pub fn fetch_all<'a>(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Result<Vec<Self>> {
        if lines.peek() != Some(&"Modules list:") {
            bail!("Invalid module list")
        };

        _ = lines.next();

        let mut modules = Vec::new();
        loop {
            if lines.peek() == Some(&"Segment list:") {
                break;
            }

            let Ok(module) = Self::fetch(lines) else {
                break;
            };

            modules.push(module);
        }

        Ok(modules)
    }

    fn fetch<'a>(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Result<Self> {
        let Some(s) = lines.peek() else {
            bail!("Invalid module")
        };

        let Some(s) = s.trim().strip_suffix(':') else {
            bail!("Invalid module")
        };

        let name = s.parse()?;
        _ = lines.next();

        let mut segments = Vec::new();
        loop {
            let Some(s) = lines.peek() else {
                break;
            };

            let Ok(segment) = s.parse::<ModuleSegment>() else {
                break;
            };

            segments.push(segment);
            _ = lines.next();
        }

        Ok(Self { name, segments })
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::util::iter_map_file_lines;
    use crate::symbols::{Module, ModuleName, ModuleSegment};
    use anyhow::Result;

    #[test]
    fn fetch_all() -> Result<()> {
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

        let mut lines = iter_map_file_lines(input);

        let modules = Module::fetch_all(&mut lines)?;
        assert_eq!(6, modules.len());

        assert_eq!(Some(&"Segment list:"), lines.peek());
        Ok(())
    }

    #[test]
    fn fetch() -> Result<()> {
        let input = r"a1basic.o:
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
";

        let mut lines = iter_map_file_lines(input);

        let module = Module::fetch(&mut lines)?;
        assert_eq!(
            ModuleName {
                name: String::from("a1basic.o"),
                path: None
            },
            module.name
        );
        assert_eq!(
            vec![ModuleSegment {
                name: String::from("A1BASIC"),
                offset: 0x000000,
                size: 0x001000,
                align: 0x00001,
                fill: 0x0000
            }],
            module.segments
        );

        let module = Module::fetch(&mut lines)?;
        assert_eq!(
            ModuleName {
                name: String::from("constants.o"),
                path: None
            },
            module.name
        );
        assert_eq!(Vec::<ModuleSegment>::new(), module.segments);

        let module = Module::fetch(&mut lines)?;
        assert_eq!(
            ModuleName {
                name: String::from("main.o"),
                path: None
            },
            module.name
        );
        assert_eq!(6, module.segments.len());

        let module = Module::fetch(&mut lines)?;
        assert_eq!(
            ModuleName {
                name: String::from("wozmon.o"),
                path: None
            },
            module.name
        );
        assert_eq!(1, module.segments.len());

        let module = Module::fetch(&mut lines)?;
        assert_eq!("copydata.o", module.name.name);
        assert!(module.name.path.is_some());
        assert_eq!(1, module.segments.len());

        let module = Module::fetch(&mut lines)?;
        assert_eq!("zeropage.o", module.name.name);
        assert!(module.name.path.is_some());
        assert_eq!(1, module.segments.len());

        assert!(lines.peek().is_none());
        assert!(Module::fetch(&mut lines).is_err());
        Ok(())
    }
}
