pub trait Bits {
    fn bits_l2r(&self) -> impl Iterator<Item = bool>;
    fn bits_r2l(&self) -> impl Iterator<Item = bool>;
}

impl Bits for u8 {
    fn bits_l2r(&self) -> impl Iterator<Item = bool> {
        U8L2RIter {
            value: *self,
            index: 0,
        }
    }

    fn bits_r2l(&self) -> impl Iterator<Item = bool> {
        U8R2LIter {
            value: *self,
            index: 0,
        }
    }
}

impl<const N: usize> Bits for [u8; N] {
    fn bits_l2r(&self) -> impl Iterator<Item = bool> {
        self.iter().rev().flat_map(Bits::bits_l2r)
    }

    fn bits_r2l(&self) -> impl Iterator<Item = bool> {
        self.iter().flat_map(Bits::bits_r2l)
    }
}

impl Bits for &[u8] {
    fn bits_l2r(&self) -> impl Iterator<Item = bool> {
        self.iter().rev().flat_map(Bits::bits_l2r)
    }

    fn bits_r2l(&self) -> impl Iterator<Item = bool> {
        self.iter().flat_map(Bits::bits_r2l)
    }
}

pub struct U8L2RIter {
    value: u8,
    index: usize,
}

impl Iterator for U8L2RIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 8 {
            self.index += 1;
            let result = self.value & 0x80 != 0;
            self.value <<= 1;
            Some(result)
        } else {
            None
        }
    }
}

pub struct U8R2LIter {
    value: u8,
    index: usize,
}

impl Iterator for U8R2LIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 8 {
            self.index += 1;
            let result = self.value & 1 != 0;
            self.value >>= 1;
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Bits;

    #[test]
    fn u8_l2r() {
        let value = 0b1001_0110;
        let bits = value.bits_l2r().collect::<Vec<_>>();
        assert_eq!(8, bits.len());
        assert!(bits[0]);
        assert!(!bits[1]);
        assert!(!bits[2]);
        assert!(bits[3]);
        assert!(!bits[4]);
        assert!(bits[5]);
        assert!(bits[6]);
        assert!(!bits[7]);
    }

    #[test]
    fn u8_r2l() {
        let value = 0b1001_0110;
        let bits = value.bits_r2l().collect::<Vec<_>>();
        assert_eq!(8, bits.len());
        assert!(!bits[0]);
        assert!(bits[1]);
        assert!(bits[2]);
        assert!(!bits[3]);
        assert!(bits[4]);
        assert!(!bits[5]);
        assert!(!bits[6]);
        assert!(bits[7]);
    }

    #[test]
    fn u8_l2r_vec() {
        let values = [0b1001_0110, 0b0110_1001];
        let bits = values.bits_l2r().collect::<Vec<_>>();
        assert_eq!(16, bits.len());
        assert!(!bits[0]);
        assert!(bits[1]);
        assert!(bits[2]);
        assert!(!bits[3]);
        assert!(bits[4]);
        assert!(!bits[5]);
        assert!(!bits[6]);
        assert!(bits[7]);
        assert!(bits[8]);
        assert!(!bits[9]);
        assert!(!bits[10]);
        assert!(bits[11]);
        assert!(!bits[12]);
        assert!(bits[13]);
        assert!(bits[14]);
        assert!(!bits[15]);
    }

    #[test]
    fn u8_r2l_vec() {
        let values = [0b1001_0110, 0b0110_1001];
        let bits = values.bits_r2l().collect::<Vec<_>>();
        assert_eq!(16, bits.len());
        assert!(!bits[0]);
        assert!(bits[1]);
        assert!(bits[2]);
        assert!(!bits[3]);
        assert!(bits[4]);
        assert!(!bits[5]);
        assert!(!bits[6]);
        assert!(bits[7]);
        assert!(bits[8]);
        assert!(!bits[9]);
        assert!(!bits[10]);
        assert!(bits[11]);
        assert!(!bits[12]);
        assert!(bits[13]);
        assert!(bits[14]);
        assert!(!bits[15]);
    }
}
