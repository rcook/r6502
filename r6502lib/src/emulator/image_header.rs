use crate::emulator::{R6502SnapshotHeader, R6502Type0Header};

pub enum ImageHeader {
    R6502Type0(R6502Type0Header),
    R6502Snapshot(R6502SnapshotHeader),
    Sim6502 { load: u16, start: u16, sp: u8 },
    Listing { load: u16, start: u16 },
    None,
}
