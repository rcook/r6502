use crate::emulator::R6502Type0Header;

pub enum ImageHeader {
    R6502Type0(R6502Type0Header),
    Sim6502 { load: u16, start: u16, sp: u8 },
    Listing { load: u16, start: u16 },
    None,
}
