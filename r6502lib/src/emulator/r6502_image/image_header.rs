use crate::_p;
use crate::emulator::r6502_image::R6502ImageType;
use crate::emulator::{Cpu, MachineTag, TotalCycles, R6502_MAGIC_NUMBER};
use anyhow::Result;
use num_traits::FromPrimitive;
use std::io::{ErrorKind, Read, Seek, Write};

pub enum ImageHeader {
    Type0 {
        machine_tag: MachineTag,
        load: u16,
        start: u16,
    },
    Snapshot {
        machine_tag: MachineTag,
        pc: u16,
        a: u8,
        x: u8,
        y: u8,
        sp: u8,
        p: u8,
        total_cycles: TotalCycles,
    },
}

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

impl ImageHeader {
    pub fn try_from_reader<R: Read + Seek>(reader: &mut R) -> Result<Option<Self>> {
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
            R6502ImageType::Type0 => Self::try_read_type0(reader)?,
            R6502ImageType::Snapshot => Self::try_read_snapshot(reader)?,
            _ => todo!(),
        })
    }

    #[must_use]
    pub const fn new_snapshot(cpu: &Cpu) -> Self {
        Self::Snapshot {
            machine_tag: cpu.bus.machine_tag(),
            pc: cpu.reg.pc,
            a: cpu.reg.a,
            x: cpu.reg.x,
            y: cpu.reg.y,
            sp: cpu.reg.sp,
            p: cpu.reg.p.bits(),
            total_cycles: cpu.total_cycles,
        }
    }

    #[must_use]
    pub const fn machine_tag(&self) -> MachineTag {
        match self {
            Self::Type0 { machine_tag, .. } | Self::Snapshot { machine_tag, .. } => *machine_tag,
        }
    }

    #[must_use]
    pub const fn load(&self) -> Option<u16> {
        match self {
            Self::Type0 { load, .. } => Some(*load),
            Self::Snapshot { .. } => None,
        }
    }

    #[must_use]
    pub const fn start(&self) -> u16 {
        match self {
            Self::Type0 { start, .. } | Self::Snapshot { pc: start, .. } => *start,
        }
    }

    #[must_use]
    pub const fn sp(&self) -> Option<u8> {
        match self {
            Self::Type0 { .. } => None,
            Self::Snapshot { sp, .. } => Some(*sp),
        }
    }

    pub const fn set_initial_cpu_state(&self, cpu: &mut Cpu) {
        match self {
            Self::Type0 { start, .. } => cpu.reg.pc = *start,
            Self::Snapshot {
                pc,
                a,
                x,
                y,
                sp,
                p,
                total_cycles,
                ..
            } => {
                cpu.reg.pc = *pc;
                cpu.reg.a = *a;
                cpu.reg.x = *x;
                cpu.reg.y = *y;
                cpu.reg.sp = *sp;
                cpu.reg.p = _p!(*p);
                cpu.total_cycles = *total_cycles;
            }
        }
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        let Self::Snapshot {
            machine_tag,
            pc,
            a,
            x,
            y,
            sp,
            p,
            total_cycles,
        } = *self
        else {
            todo!();
        };

        writer.write_all(&machine_tag)?;
        writer.write_all(&pc.to_le_bytes())?;
        writer.write_all(&[a, x, y, sp, p])?;
        writer.write_all(&total_cycles.to_le_bytes())?;
        Ok(())
    }

    fn try_read_type0<R: Read + Seek>(reader: &mut R) -> Result<Option<Self>> {
        let mut machine_tag = MachineTag::default();
        fill_buffer!(reader, &mut machine_tag);
        let load = read_le_word!(reader);
        let start = read_le_word!(reader);
        Ok(Some(Self::Type0 {
            machine_tag,
            load,
            start,
        }))
    }

    fn try_read_snapshot<R: Read + Seek>(reader: &mut R) -> Result<Option<Self>> {
        let mut machine_tag = MachineTag::default();
        fill_buffer!(reader, &mut machine_tag);
        let pc = read_le_word!(reader);
        let a = read_byte!(reader);
        let x = read_byte!(reader);
        let y = read_byte!(reader);
        let sp = read_byte!(reader);
        let p = read_byte!(reader);
        let total_cycles = read_le_quad_word!(reader);
        Ok(Some(Self::Snapshot {
            machine_tag,
            pc,
            a,
            x,
            y,
            sp,
            p,
            total_cycles,
        }))
    }
}
