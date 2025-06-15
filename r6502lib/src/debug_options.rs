use std::path::PathBuf;

pub struct DebugOptions {
    pub path: PathBuf,
    pub load: Option<u16>,
    pub start: Option<u16>,
    pub machine: Option<String>,
}
