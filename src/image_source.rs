use crate::{make_word, ImageInfo, Memory, SymbolInfo};
use anyhow::{bail, Result};
use std::fs::File;
use std::io::{Cursor, ErrorKind, Read, Seek};
use std::path::{Path, PathBuf};

const MAGIC_NUMBER: u16 = 0x6502u16;

pub(crate) enum ImageSource<'a> {
    File(PathBuf, Option<u16>, Option<u16>),
    Bytes(&'a [u8], Option<u16>, Option<u16>),
}

impl<'a> ImageSource<'a> {
    pub(crate) fn from_file(
        path: &Path,
        default_origin: Option<u16>,
        default_start: Option<u16>,
    ) -> Self {
        Self::File(path.to_path_buf(), default_origin, default_start)
    }

    #[allow(unused)]
    pub(crate) fn from_bytes(
        bytes: &'a [u8],
        default_origin: Option<u16>,
        default_start: Option<u16>,
    ) -> Self {
        Self::Bytes(bytes, default_origin, default_start)
    }

    pub(crate) fn load_into_memory(&self, memory: &mut Memory) -> Result<ImageInfo> {
        match self {
            Self::File(path, default_origin, default_start) => {
                let mut file = File::open(path)?;
                let (origin, start) = Self::read_header(&mut file, default_origin, default_start)?;
                let len = memory.len();
                let buffer = &mut memory[origin as usize..len];
                match file.read_exact(buffer) {
                    Ok(()) => {}
                    Err(e) if e.kind() == ErrorKind::UnexpectedEof => {}
                    Err(e) => bail!(e),
                }

                let mut file_name = path.file_name().expect("Must succeed").to_os_string();
                file_name.push(".json");
                let symbol_path = path.parent().expect("Must succeed").join(file_name);

                Ok(if symbol_path.is_file() {
                    let symbol_file = File::open(symbol_path)?;
                    let symbols = serde_json::from_reader::<_, Vec<SymbolInfo>>(symbol_file)?;
                    ImageInfo { start, symbols }
                } else {
                    ImageInfo {
                        start,
                        symbols: Vec::new(),
                    }
                })
            }
            Self::Bytes(bytes, default_origin, default_start) => {
                let mut cursor = Cursor::new(bytes);
                let (origin, start) =
                    Self::read_header(&mut cursor, default_origin, default_start)?;
                let origin_idx = origin as usize;
                let count = (bytes.len() - cursor.position() as usize)
                    .min(memory.len().checked_sub(origin_idx).unwrap());
                let buffer = &mut memory[origin_idx..origin_idx + count];
                match cursor.read_exact(buffer) {
                    Ok(()) => {}
                    Err(e) if e.kind() == ErrorKind::UnexpectedEof => {}
                    Err(e) => bail!(e),
                }
                Ok(ImageInfo {
                    start,
                    symbols: Vec::new(),
                })
            }
        }
    }

    fn read_header<R: Read + Seek>(
        reader: &mut R,
        default_origin: &Option<u16>,
        default_start: &Option<u16>,
    ) -> Result<(u16, u16)> {
        let mut header = [0x00u8; 6];
        match reader.read_exact(&mut header) {
            Ok(()) => {
                let magic_number = make_word(header[1], header[0]);
                if magic_number == MAGIC_NUMBER {
                    let origin = make_word(header[3], header[2]);
                    let start = make_word(header[5], header[4]);
                    Ok((origin, start))
                } else {
                    reader.rewind()?;
                    Ok((
                        default_origin.unwrap_or(0x0000u16),
                        default_start.unwrap_or(0x0000u16),
                    ))
                }
            }
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                reader.rewind()?;
                Ok((
                    default_origin.unwrap_or(0x0000u16),
                    default_start.unwrap_or(0x0000u16),
                ))
            }
            Err(e) => bail!(e),
        }
    }
}
