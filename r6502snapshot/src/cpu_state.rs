use r6502core::TotalCycles;

#[derive(Debug)]
pub struct CpuState {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub p: u8,
    pub total_cycles: TotalCycles,
}
