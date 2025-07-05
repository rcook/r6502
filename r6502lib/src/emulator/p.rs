use bitflags::bitflags;
use std::fmt::{Display, Formatter, Result as StdResult};

const P_STR: &str = "NV-BDIZC";

// TBD: Consider using bitvec (https://docs.rs/bitvec/0.22.3/bitvec/) for this
// Reference: https://www.nesdev.org/wiki/Status_flags
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct P: u8 {
        const N =           0b1000_0000; // Negative
        const V =           0b0100_0000; // Overflow
        const ALWAYS_ONE =  0b0010_0000; // Always 1
        const B =           0b0001_0000; // B
        const D =           0b0000_1000; // Decimal
        const I =           0b0000_0100; // Interrupt Disable
        const Z =           0b0000_0010; // Zero
        const C =           0b0000_0001; // Carry
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
        let mut mask = 0b1000_0000;
        let value = self.bits();
        write!(f, "[")?;
        for c in P_STR.chars() {
            if c == '-' {
                write!(f, "-")?;
            } else if (value & mask) == 0 {
                write!(f, ".")?;
            } else {
                write!(f, "{c}", c = c.to_uppercase())?;
            }
            mask >>= 1;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[macro_export]
macro_rules! _p {
    ($value: expr) => {
        $crate::emulator::P::from_bits($value).unwrap()
    };
}

#[macro_export]
macro_rules! p {
    () => {
        $crate::emulator::P::empty()
    };
    ($flag: ident) => {
        $crate::emulator::P::$flag
    };
    ($flag: ident, $($flags: ident), +) => {
        $crate::p!($flag) | $crate::p!($($flags), +)
    };
}

#[macro_export]
macro_rules! p_get {
    ($reg: expr, $flag: ident) => {
        $reg.p.contains($crate::emulator::P::$flag)
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
        $reg.p.set($crate::emulator::P::$flag, $value)
    };
}

#[cfg(test)]
mod tests {
    use crate::emulator::P;
    use rstest::rstest;

    #[rstest]
    #[case("[..-.....]", _p!(0b0000_0000))]
    #[case("[NV-BDIZC]", _p!(0b1111_1111))]
    #[case("[NV-BDIZC]", _p!(0b1101_1111))]
    #[case("[NV-BD..C]", _p!(0b1101_1001))]
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
            P::from_bits(0b1110_1101)
        );
    }
}
