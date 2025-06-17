use strum_macros::EnumString;

#[derive(Debug, PartialEq)]
pub enum ExportKind {
    Label,
    Constant,
}
