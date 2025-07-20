pub trait Wrap<T> {
    fn wrap(value: T) -> Self;
}

impl Wrap<u8> for i8 {
    #[allow(clippy::cast_possible_wrap)]
    fn wrap(value: u8) -> Self {
        value as Self
    }
}
