use crate::emulator::MachineTag;

pub enum ImageHeader {
    R6502Type0 {
        machine_tag: MachineTag,
        load: u16,
        start: u16,
    },
    Sim6502 {
        load: u16,
        start: u16,
        sp: u8,
    },
    Listing {
        load: u16,
        start: u16,
    },
    None,
}
