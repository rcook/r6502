use crate::{CpuState, SIM6502_MAGIC_NUMBER};
use anyhow::{Result, bail};
use r6502core::util::make_word;
use std::io::{ErrorKind, Read, Seek};

pub enum OtherImageHeader {
    Sim6502 { load: u16, start: u16, sp: u8 },
    Listing { load: u16, start: u16 },
    Raw,
}

impl OtherImageHeader {
    pub fn from_reader<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        Ok(Self::try_read_sim6502(reader)?.unwrap_or(Self::Raw))
    }

    #[must_use]
    pub const fn get_initial_cpu_state(&self, _reset_addr: u16) -> CpuState {
        match self {
            Self::Sim6502 { start, sp, .. } => CpuState {
                pc: *start,
                a: 0x00,
                x: 0x00,
                y: 0x00,
                sp: *sp,
                p: 0x00,
                total_cycles: 0,
            },
            Self::Listing { start, .. } => CpuState {
                pc: *start,
                a: 0x00,
                x: 0x00,
                y: 0x00,
                sp: 0x00,
                p: 0x00,
                total_cycles: 0,
            },
            Self::Raw { .. } => CpuState {
                pc: 0x0000,
                a: 0x00,
                x: 0x00,
                y: 0x00,
                sp: 0x00,
                p: 0x00,
                total_cycles: 0,
            },
        }
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
