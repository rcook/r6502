#[derive(Clone, Copy)]
#[repr(u8)]
pub enum MachineType {
    None = 0,
    Sim6502 = 1,
    Acorn = 2,
    Apple1 = 3,
}
