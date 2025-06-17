#[derive(Debug, PartialEq)]
pub enum AddressSize {
    ZeroPage,
    Absolute,
    Far,
    Long,
}
