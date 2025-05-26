#[derive(Clone, Copy, Debug)]
pub(crate) enum AddressingMode {
    Absolute,
    AbsoluteX,
    Immediate,
    Implied,
    Relative,
}
