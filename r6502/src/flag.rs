// NV1BDIZC
#[repr(u8)]
pub(crate) enum Flag {
    N = 0b1000_0000u8,
    #[allow(unused)]
    V = 0b0100_0000u8,
    B = 0b0001_0000u8,
    #[allow(unused)]
    D = 0b0000_1000u8,
    #[allow(unused)]
    I = 0b0000_0100u8,
    Z = 0b0000_0010u8,
    Carry = 0b0000_0001u8,
}
