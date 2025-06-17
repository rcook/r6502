use crate::emulator::{ImageHeader, MachineTag, R6502ImageType, R6502_MAGIC_NUMBER};
use anyhow::Result;
use num_traits::FromPrimitive;
use std::io::{ErrorKind, Read, Seek};

pub struct R6502Type0Header {
    pub machine_tag: MachineTag,
    pub load: u16,
    pub start: u16,
}

pub fn read_r6502_image_header<R: Read + Seek>(reader: &mut R) -> Result<Option<ImageHeader>> {
    macro_rules! fill_buffer {
        ($reader: expr, $buffer: expr) => {
            match $reader.read_exact($buffer) {
                Ok(()) => {}
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                    $reader.rewind()?;
                    return Ok(None);
                }
                Err(e) => anyhow::bail!(e),
            }
        };
    }

    macro_rules! read_le_word {
        ($reader: expr) => {{
            let mut bytes = [0x00; 2];
            fill_buffer!($reader, &mut bytes);
            u16::from_le_bytes(bytes)
        }};
    }

    macro_rules! read_byte {
        ($reader: expr) => {{
            let mut bytes = [0x00; 1];
            fill_buffer!($reader, &mut bytes);
            bytes[0]
        }};
    }

    fn read_type0_header<R: Read + Seek>(reader: &mut R) -> Result<Option<ImageHeader>> {
        let mut machine_tag = MachineTag::default();
        fill_buffer!(reader, &mut machine_tag);
        let load = read_le_word!(reader);
        let start = read_le_word!(reader);
        Ok(Some(ImageHeader::R6502Type0(R6502Type0Header {
            machine_tag,
            load,
            start,
        })))
    }

    let magic_number = read_le_word!(reader);
    if magic_number != R6502_MAGIC_NUMBER {
        reader.rewind()?;
        return Ok(None);
    }

    let Some(image_type) = R6502ImageType::from_u8(read_byte!(reader)) else {
        reader.rewind()?;
        return Ok(None);
    };

    Ok(match image_type {
        R6502ImageType::Type0 => read_type0_header(reader)?,
        _ => todo!(),
    })
}
