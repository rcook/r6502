use crate::SymbolInfo;

pub(crate) struct ImageInfo {
    pub(crate) start: u16,

    #[allow(unused)]
    pub(crate) symbols: Vec<SymbolInfo>,
}
