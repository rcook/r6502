use crate::{make_word, ImageStartInfo, Memory};
use anyhow::{bail, Result};
use std::ffi::OsString;
use std::fs::File;
use std::io::{ErrorKind, Read, Seek, Write};
use std::path::{Path, PathBuf};

const MAGIC_NUMBER: u16 = 0x6502u16;

pub(crate) struct ImageInfo {
    path: PathBuf,
    default_origin: Option<u16>,
    start: Option<u16>,
}

impl ImageInfo {
    pub(crate) fn new(path: &Path, origin: Option<u16>, start: Option<u16>) -> Self {
        Self {
            path: path.to_path_buf(),
            default_origin: origin,
            start,
        }
    }

    pub(crate) fn load(&self, memory: &mut Memory) -> Result<ImageStartInfo> {
        let mut file = File::open(&self.path)?;
        let (origin, start) = self.read_header(&mut file)?;
        let len = memory.len();
        let buffer = &mut memory[origin as usize..len];
        match file.read_exact(buffer) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {}
            Err(e) => bail!(e),
        }
        Ok(ImageStartInfo { start })
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

    fn read_header(&self, file: &mut File) -> Result<(u16, u16)> {
        let mut header = [0x00u8; 6];
        match file.read_exact(&mut header) {
            Ok(()) => {
                let magic_number = make_word(header[1], header[0]);
                if magic_number == MAGIC_NUMBER {
                    let origin = make_word(header[3], header[2]);
                    let start = make_word(header[5], header[4]);
                    Ok((origin, start))
                } else {
                    file.rewind()?;
                    Ok((
                        self.default_origin.unwrap_or(0x0000u16),
                        self.start.unwrap_or(0x0000u16),
                    ))
                }
            }
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                file.rewind()?;
                Ok((
                    self.default_origin.unwrap_or(0x0000u16),
                    self.start.unwrap_or(0x0000u16),
                ))
            }
            Err(e) => bail!(e),
        }
    }
}
