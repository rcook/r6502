use crate::P;

const DEFAULT_S: u8 = 0xff;

#[derive(Debug, PartialEq)]
pub(crate) struct Reg {
    pub(crate) a: u8,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) p: P,
    pub(crate) pc: u16,
    pub(crate) s: u8,
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

#[allow(unused)]
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
