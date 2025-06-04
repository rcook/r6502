use bitflags::bitflags;
use std::fmt::{Display, Formatter, Result as StdResult};

const P_STR: &str = "NVXBDIZC";

// TBD: Consider using bitvec (https://docs.rs/bitvec/0.22.3/bitvec/) for this
// Reference: https://www.nesdev.org/wiki/Status_flags
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct P: u8 {
        const N =           0b10000000; // Negative
        const V =           0b01000000; // Overflow
        const ALWAYS_ONE =  0b00100000; // Always 1
        const B =           0b00010000; // B
        const D =           0b00001000; // Decimal
        const I =           0b00000100; // Interrupt Disable
        const Z =           0b00000010; // Zero
        const C =           0b00000001; // Carry
    }
}

impl Default for P {
    fn default() -> Self {
        // TBD: What is the initial state of the P register?
        Self::empty()
    }
}

impl Display for P {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdResult {
        let mut mask = 0b10000000;
        let value = self.bits();
        for c in P_STR.chars() {
            if (value & mask) == 0 {
                write!(f, "{c}", c = c.to_lowercase())?
            } else {
                write!(f, "{c}")?
            }
            mask >>= 1;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! _p {
    ($value: expr) => {
        $crate::P::from_bits($value).unwrap()
    };
}

#[cfg(test)]
macro_rules! p {
    () => {
        $crate::P::empty()
    };
    ($flag: ident) => {
        $crate::P::$flag
    };
    ($flag: ident, $($flags: ident), +) => {
        $crate::p!($flag) | $crate::p!($($flags), +)
    };
}

#[cfg(test)]
pub(crate) use p;

#[macro_export]
macro_rules! p_get {
    ($reg: expr, $flag: ident) => {
        $reg.p.contains($crate::P::$flag)
    };
}

#[macro_export]
macro_rules! p_value {
    ($reg: expr, $flag: ident) => {
        if $crate::p_get!($reg, $flag) {
            1
        } else {
            0
        }
    };
}

#[macro_export]
macro_rules! p_set {
    ($reg: expr, $flag: ident, $value: expr) => {
        $reg.p.set($crate::P::$flag, $value)
    };
}

#[cfg(test)]
mod tests {
    use crate::P;
    use rstest::rstest;

    #[rstest]
    #[case("nvxbdizc", _p!(0b00000000))]
    #[case("NVXBDIZC", _p!(0b11111111))]
    #[case("NVxBDIZC", _p!(0b11011111))]
    #[case("NVxBDizC", _p!(0b11011001))]
    fn display(#[case] expected: &str, #[case] input: P) {
        assert_eq!(expected, input.to_string());
    }

    #[test]
    fn basics() {
        assert_eq!(P::empty(), P::default());
        assert_eq!(P::empty(), p!());
        assert_eq!(P::N, p!(N));
        assert_eq!(P::N | P::V, p!(N, V));
    }

    #[test]
    fn from_bits() {
        assert_eq!(
            Some(P::N | P::V | P::ALWAYS_ONE | P::D | P::I | P::C),
            P::from_bits(0b11101101)
        )
    }
}
