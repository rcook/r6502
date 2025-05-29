use crate::P;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Reg {
    pub(crate) a: u8,
    pub(crate) pc: u16,
    pub(crate) p: P,
}

#[allow(unused)]
macro_rules! reg {
    ($a: expr, $pc: expr) => {
        $crate::Reg {
            a: $a,
            pc: $pc,
            p: $crate::P::empty()
        }
    };
    ($a: expr, $pc: expr, $($flags: ident), *) => {
        $crate::Reg {
            a: $a,
            pc: $pc,
            p: $crate::p!($($flags), *)
        }
    };
}

pub(crate) use reg;

#[cfg(test)]
mod tests {
    use crate::{Reg, P};

    #[test]
    fn basics() {
        assert_eq!(
            Reg {
                a: 0x12,
                pc: 0x0000,
                p: P::default()
            },
            reg!(0x12, 0x0000)
        );
        assert_eq!(
            Reg {
                a: 0x23,
                pc: 0x1000,
                p: P::N
            },
            reg!(0x23, 0x1000, N)
        );
        assert_eq!(
            Reg {
                a: 0x34,
                pc: 0x2000,
                p: P::N | P::Z,
            },
            reg!(0x34, 0x2000, N, Z)
        );
    }
}
