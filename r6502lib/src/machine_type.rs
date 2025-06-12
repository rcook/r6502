#[derive(Clone, Copy)]
#[repr(u8)]
pub enum MachineType {
    AllRam = 0,
    Custom = 10,
    Acorn = 30,
    Apple1 = 40,
}
