use crate::{_p, P};
use derive_builder::Builder;

const DEFAULT_S: u8 = 0xff;

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Reg {
    #[builder(default = 0x00)]
    pub a: u8,

    #[builder(default = 0x00)]
    pub x: u8,

    #[builder(default = 0x00)]
    pub y: u8,

    #[builder(default = _p!(0b00110000))]
    pub p: P,

    #[builder(default = 0x0000)]
    pub pc: u16,

    #[builder(default = 0xff)]
    pub s: u8,
}

impl Reg {
    pub fn display(&self) -> String {
        format!(
            "pc={:04X} NV1BDIZC={:08b} a={:02X} x={:02X} y={:02X} s={:02X}",
            self.pc, self.p, self.a, self.x, self.y, self.s,
        )
    }
}

impl Default for Reg {
    fn default() -> Self {
        Self {
            a: u8::default(),
            x: u8::default(),
            y: u8::default(),
            p: P::default(),
            pc: u16::default(),
            s: DEFAULT_S,
        }
    }
}

#[cfg(test)]
macro_rules! reg {
    ($a: expr, $pc: expr) => {
        $crate::Reg {
            a: $a,
            pc: $pc,
            ..Default::default()
        }
    };
    ($a: expr, $pc: expr, $($flags: ident), *) => {
        $crate::Reg {
            a: $a,
            pc: $pc,
            p: $crate::p!($($flags), *),
            ..Default::default()
        }
    };
}

#[cfg(test)]
pub(crate) use reg;

#[cfg(test)]
mod tests {
    use crate::reg::DEFAULT_S;
    use crate::{Reg, P};

    #[test]
    fn basics() {
        assert_eq!(
            Reg {
                a: 0x12,
                x: 0x00,
                y: 0x00,
                p: P::default(),
                pc: 0x0000,
                s: DEFAULT_S,
            },
            reg!(0x12, 0x0000)
        );
        assert_eq!(
            Reg {
                a: 0x23,
                x: 0x00,
                y: 0x00,
                p: P::N,
                pc: 0x1000,
                s: DEFAULT_S,
            },
            reg!(0x23, 0x1000, N)
        );
        assert_eq!(
            Reg {
                a: 0x34,
                x: 0x00,
                y: 0x00,
                p: P::N | P::Z,
                pc: 0x2000,
                s: DEFAULT_S,
            },
            reg!(0x34, 0x2000, N, Z)
        );
    }
}
