pub trait Truncate<T> {
    fn truncate(value: T) -> Self;
}

impl Truncate<i32> for u8 {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn truncate(value: i32) -> Self {
        value as Self
    }
}

impl Truncate<u16> for i8 {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn truncate(value: u16) -> Self {
        value as Self
    }
}

impl Truncate<u16> for u8 {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn truncate(value: u16) -> Self {
        value as Self
    }
}

impl Truncate<i32> for u16 {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn truncate(value: i32) -> Self {
        value as Self
    }
}
