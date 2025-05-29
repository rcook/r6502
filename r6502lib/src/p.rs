use bitflags::bitflags;

pub(crate) const P_STR: &str = "NV1BDIZC";

// Reference: https://www.nesdev.org/wiki/Status_flags
bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub(crate) struct P: u8 {
        const N = 0b1000000; // Negative
        const V = 0b0100000; // Overflow
        const ONE = 0b00100000; // Always 1
        const B = 0b00010000; // B
        const D = 0b00001000; // Decimal
        const I = 0b00000100; // Interrupt Disable
        const Z = 0b00000010; // Zero
        const C = 0b00000001; // Carry
    }
}

impl Default for P {
    fn default() -> Self {
        // TBD: What is the initial state of the P register?
        Self::empty()
    }
}

#[allow(unused)]
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

pub(crate) use p;

macro_rules! get {
    ($reg: expr, $flag: ident) => {
        $reg.p.contains($crate::P::$flag)
    };
}

pub(crate) use get;

macro_rules! value {
    ($reg: expr, $flag: ident) => {
        if $crate::get!($reg, $flag) {
            1
        } else {
            0
        }
    };
}

pub(crate) use value;

macro_rules! set {
    ($reg: expr, $flag: ident, $value: expr) => {
        $reg.p.set($crate::P::$flag, $value)
    };
}

pub(crate) use set;

#[cfg(test)]
mod tests {
    use crate::P;

    #[test]
    fn basics() {
        assert_eq!(P::empty(), P::default());
        assert_eq!(P::empty(), p!());
        assert_eq!(P::N, p!(N));
        assert_eq!(P::N | P::V, p!(N, V));
    }
}
