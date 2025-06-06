pub const MEMORY_SIZE: usize = 0x10000;
pub const STACK_BASE: u16 = 0x0100;
#[allow(unused)]
pub const NMI: u16 = 0xfffa;
pub const RESET: u16 = 0xfffc;
pub const IRQ: u16 = 0xfffe;

pub const IRQ_ADDR: u16 = 0xdeadu16;
pub const OSHALT: u16 = 0xe000;
pub const OSWRCH: u16 = 0xffee;

pub(crate) const R6502_MAGIC_NUMBER: u16 = 0x6502;
pub(crate) const SIM6502_MAGIC_NUMBER: &str = "sim65";
pub(crate) const DEFAULT_LOAD: u16 = 0x0000;
pub(crate) const DEFAULT_START: u16 = 0x0000;
pub(crate) const DEFAULT_SP: u8 = 0xff;

// Apple 1 PIA addresses etc.
#[allow(unused)]
pub(crate) const KBD: u16 = 0xD010; // PIA.A keyboard input
#[allow(unused)]
pub(crate) const KBDCR: u16 = 0xD011; // PIA.A keyboard control register
#[allow(unused)]
pub(crate) const DSP: u16 = 0xD012; // PIA.B display output register
#[allow(unused)]
pub(crate) const DSPCR: u16 = 0xD013; //  PIA.B display control register
