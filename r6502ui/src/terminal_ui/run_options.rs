use std::path::PathBuf;

pub struct RunOptions {
    pub path: PathBuf,
    pub load: Option<u16>,
    pub start: Option<u16>,
    pub trace: bool,
    pub cycles: bool,
    pub stop_after: Option<u64>,
    pub machine: Option<String>,
}
