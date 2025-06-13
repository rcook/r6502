pub const MEMORY_SIZE: usize = 0x10000;
pub const STACK_BASE: u16 = 0x0100;
pub const NMI: u16 = 0xfffa;
pub const RESET: u16 = 0xfffc;
pub const IRQ: u16 = 0xfffe;

pub const IRQ_ADDR: u16 = 0xdeadu16;

pub(crate) const R6502_MAGIC_NUMBER: u16 = 0x0265; // "65 02" in hexdump
pub(crate) const R6502_DUMP_MAGIC_NUMBERS: [u8; 4] = [b'D', b'U', b'M', b'P'];
pub(crate) const SIM6502_MAGIC_NUMBER: &str = "sim65";
pub(crate) const DEFAULT_LOAD: u16 = 0x0000;
pub(crate) const DEFAULT_START: u16 = 0x0000;
pub(crate) const DEFAULT_SP: u8 = 0xff;
