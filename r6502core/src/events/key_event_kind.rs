#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyEventKind {
    Press,
    Repeat,
    Release,
}
