use anyhow::{bail, Result};
use std::io::{ErrorKind, Read, Seek};

use crate::emulator::{util::make_word, SIM6502_MAGIC_NUMBER};

pub enum OtherImageHeader {
    Sim6502 { load: u16, start: u16, sp: u8 },
    Listing { load: u16, start: u16 },
    Raw,
}

impl OtherImageHeader {
    pub fn from_reader<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        Ok(Self::try_read_sim6502(reader)?.unwrap_or(Self::Raw))
    }

    fn try_read_sim6502<R: Read + Seek>(reader: &mut R) -> Result<Option<Self>> {
        let mut header = [0x00u8; 12];
        match reader.read_exact(&mut header) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                reader.rewind()?;
                return Ok(None);
            }
            Err(e) => bail!(e),
        }

        let bytes = SIM6502_MAGIC_NUMBER.as_bytes();
        assert_eq!(5, bytes.len());

        // "sim65" header
        if *bytes != header[0..bytes.len()] {
            reader.rewind()?;
            return Ok(None);
        }

        // Version number
        if header[5] != 2 {
            reader.rewind()?;
            return Ok(None);
        }

        // CPU version
        if header[6] != 0 {
            reader.rewind()?;
            return Ok(None);
        }

        // Initial stack pointer
        let sp = header[7];
        assert_eq!(0xff, sp);

        // Load address
        let load = make_word(header[9], header[8]);

        // Start address
        let start = make_word(header[11], header[10]);

        Ok(Some(Self::Sim6502 { load, start, sp }))
    }
}
