#[macro_export]
macro_rules! f32 {
    ($value: expr) => {
        conv2::ConvAsUtil::<f32>::approx($value)
    };
}

#[macro_export]
macro_rules! u8 {
    ($value: expr) => {
        conv2::ConvAsUtil::<u8>::approx($value)
    };
}

#[macro_export]
macro_rules! u32 {
    ($value: expr) => {
        conv2::ConvAsUtil::<u32>::approx($value)
    };
}
