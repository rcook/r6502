use crate::emulator::{
    r6502_image::{
        image_header::{R6502SnapshotHeader, R6502Type0Header},
        image_type::R6502ImageType,
    },
    ImageHeader, MachineTag, R6502_MAGIC_NUMBER,
};
use anyhow::Result;
use num_traits::FromPrimitive;
use std::io::{ErrorKind, Read, Seek};

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

    macro_rules! read_byte {
        ($reader: expr) => {{
            let mut bytes = [0x00; 1];
            fill_buffer!($reader, &mut bytes);
            bytes[0]
        }};
    }

    macro_rules! read_le_word {
        ($reader: expr) => {{
            let mut bytes = [0x00; 2];
            fill_buffer!($reader, &mut bytes);
            u16::from_le_bytes(bytes)
        }};
    }

    macro_rules! read_le_quad_word {
        ($reader: expr) => {{
            let mut bytes = [0x00; 8];
            fill_buffer!($reader, &mut bytes);
            u64::from_le_bytes(bytes)
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

    fn read_snapshot_header<R: Read + Seek>(reader: &mut R) -> Result<Option<ImageHeader>> {
        let mut machine_tag = MachineTag::default();
        fill_buffer!(reader, &mut machine_tag);
        let pc = read_le_word!(reader);
        let a = read_byte!(reader);
        let x = read_byte!(reader);
        let y = read_byte!(reader);
        let sp = read_byte!(reader);
        let p = read_byte!(reader);
        let total_cycles = read_le_quad_word!(reader);
        Ok(Some(ImageHeader::R6502Snapshot(R6502SnapshotHeader {
            machine_tag,
            pc,
            a,
            x,
            y,
            sp,
            p,
            total_cycles,
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
        R6502ImageType::Snapshot => read_snapshot_header(reader)?,
        _ => todo!(),
    })
}
