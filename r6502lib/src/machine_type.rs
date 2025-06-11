#[derive(Clone, Copy)]
#[repr(u8)]
pub enum MachineType {
    None = 0,
    Custom = 10,
    Sim6502 = 20,
    Acorn = 30,
    Apple1 = 40,
}
