use crate::emulator::{MachineTag, TotalCycles};

pub struct R6502Type0Header {
    pub machine_tag: MachineTag,
    pub load: u16,
    pub start: u16,
}

pub struct R6502SnapshotHeader {
    pub machine_tag: MachineTag,
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub p: u8,
    pub total_cycles: TotalCycles,
}
