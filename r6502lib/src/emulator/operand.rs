#[derive(Clone, Debug)]
pub enum Operand {
    None,
    Byte(u8),
    Word(u16),
}
