use crate::Memory;
use anyhow::{bail, Result};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};

pub(crate) struct ProgramInfo {
    path: PathBuf,
    start: u16,
}

impl ProgramInfo {
    pub(crate) fn new(path: &Path, start: u16) -> Self {
        Self {
            path: path.to_path_buf(),
            start,
        }
    }

    pub(crate) fn start(&self) -> u16 {
        self.start
    }

    pub(crate) fn load(&self, memory: &mut Memory) -> Result<()> {
        let len = memory.len();
        let buffer = &mut memory[self.start as usize..len];
        let mut file = File::open(&self.path)?;
        match file.read_exact(buffer) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {}
            Err(e) => bail!(e),
        }
        Ok(())
    }
}
