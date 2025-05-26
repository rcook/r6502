use crate::Cpu;

pub(crate) type Memory = [u8; 0x10000];
pub(crate) type OpFn = fn(&mut Cpu) -> ();
