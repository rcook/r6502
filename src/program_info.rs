use crate::Memory;
use anyhow::{bail, Result};
use std::ffi::OsString;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
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

    pub(crate) fn save_dump(&self, memory: &Memory) -> Result<()> {
        let dir = self.path.parent().expect("Must succeed");
        let file_name = self.path.file_name().expect("Must succeed");
        let mut output_file_name = OsString::new();
        output_file_name.push(file_name);
        output_file_name.push(".dump");
        let output_path = dir.join(output_file_name);
        let mut file = File::create(output_path)?;
        file.write_all(memory)?;
        Ok(())
    }
}
