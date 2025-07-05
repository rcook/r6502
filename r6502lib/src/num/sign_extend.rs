pub trait SignExtend<T> {
    fn sign_extend(value: T) -> Self;
}

impl SignExtend<u8> for u16 {
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_sign_loss)]
    fn sign_extend(value: u8) -> Self {
        (value as i8) as Self
    }
}
