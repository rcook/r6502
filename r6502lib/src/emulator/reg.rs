use crate::emulator::P;
use std::fmt::{Display, Formatter, Result as FmtResult};

const DEFAULT_SP: u8 = 0xff;

#[derive(Clone, Debug, PartialEq)]
pub struct Reg {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: P,
    pub pc: u16,
    pub sp: u8,
}

impl Default for Reg {
    fn default() -> Self {
        Self {
            a: u8::default(),
            x: u8::default(),
            y: u8::default(),
            p: P::default(),
            pc: u16::default(),
            sp: DEFAULT_SP,
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "PC:{pc:04X} A:{a:02X} X:{x:02X} Y:{y:02X} S:{sp:02X} {p}",
            pc = self.pc,
            a = self.a,
            x = self.x,
            y = self.y,
            sp = self.sp,
            p = self.p,
        )
    }
}
