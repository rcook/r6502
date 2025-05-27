#[derive(Clone, Copy, Debug)]
pub(crate) enum AddressingMode {
    Absolute,
    AbsoluteX,
    Immediate,
    Implied,
    IndirectIndexedY,
    Relative,
    ZeroPage,
}
