use crate::emulator::r6502_image::R6502ImageType;
use crate::emulator::{Cpu, CpuState, R6502_MAGIC_NUMBER};
use anyhow::{Result, bail};
use num_traits::FromPrimitive;
use r6502core::MachineTag;
use r6502core::util::make_word;
use r6502cpu::TotalCycles;
use r6502cpu::constants::RESET;
use std::io::{ErrorKind, Read, Seek};

pub enum ImageHeader {
    // A module represents a program or subroutine that runs independently
    // of an operating system. The module will be loaded at the load address
    // and execution will begin at the start address.
    Module {
        machine_tag: MachineTag,
        load: u16,
        start: u16,
    },

    // A snapshot captures the full state of execution of the machine
    // at a point in time including all addressable memory and all CPU
    // registers.
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

    // A system snapshot captures the full initial state of the
    // machine and is useful for providing a system's operating
    // system in ROM. The system snapshot will be loaded at the
    // specified load address and will be stared from the 6502
    // RESET vector. It's important that the system snapshot
    // include valid NMI, RESET and IRQ vectors at the top of
    // memory.
    System {
        machine_tag: MachineTag,
        load: u16,
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
        let pos = reader.stream_position()?;
        assert_eq!(0, pos);

        let magic_number = read_le_word!(reader);
        if magic_number != R6502_MAGIC_NUMBER {
            reader.rewind()?;
            return Ok(None);
        }

        let Some(image_type) = R6502ImageType::from_u8(read_byte!(reader)) else {
            reader.rewind()?;
            return Ok(None);
        };

        let result = match image_type {
            R6502ImageType::Module => Self::try_read_module(reader)?,
            R6502ImageType::Snapshot => Self::try_read_snapshot(reader)?,
            R6502ImageType::System => Self::try_read_system(reader)?,
        };

        if result.is_some() {
            let header_len = reader.stream_position()? - pos;
            if header_len != image_type.header_len() {
                bail!("invalid header")
            }
        }

        Ok(result)
    }

    #[must_use]
    pub const fn machine_tag(&self) -> MachineTag {
        match self {
            Self::Module { machine_tag, .. }
            | Self::Snapshot { machine_tag, .. }
            | Self::System { machine_tag, .. } => *machine_tag,
        }
    }

    #[must_use]
    pub const fn load(&self) -> Option<u16> {
        match self {
            Self::Module { load, .. } | Self::System { load, .. } => Some(*load),
            Self::Snapshot { .. } => None,
        }
    }

    #[must_use]
    pub const fn start(&self) -> Option<u16> {
        match self {
            Self::Module { start, .. } | Self::Snapshot { pc: start, .. } => Some(*start),
            Self::System { .. } => None,
        }
    }

    #[must_use]
    pub const fn sp(&self) -> Option<u8> {
        match self {
            Self::Module { .. } | Self::System { .. } => None,
            Self::Snapshot { sp, .. } => Some(*sp),
        }
    }

    #[must_use]
    pub fn get_initial_cpu_state(&self, cpu: &Cpu) -> CpuState {
        match self {
            Self::Module { start, .. } => CpuState {
                pc: *start,
                a: 0x00,
                x: 0x00,
                y: 0x00,
                sp: 0x00,
                p: 0x00,
                total_cycles: 0,
            },
            Self::Snapshot {
                pc,
                a,
                x,
                y,
                sp,
                p,
                total_cycles,
                ..
            } => CpuState {
                pc: *pc,
                a: *a,
                x: *x,
                y: *y,
                sp: *sp,
                p: *p,
                total_cycles: *total_cycles,
            },
            Self::System { .. } => {
                let reset_lo = cpu.bus.load(RESET);
                let reset_hi = cpu.bus.load(RESET.wrapping_add(1));
                let reset = make_word(reset_hi, reset_lo);
                CpuState {
                    pc: reset,
                    a: 0x00,
                    x: 0x00,
                    y: 0x00,
                    sp: 0x00,
                    p: 0x00,
                    total_cycles: 0,
                }
            }
        }
    }

    fn try_read_module<R: Read + Seek>(reader: &mut R) -> Result<Option<Self>> {
        let mut machine_tag = MachineTag::default();
        fill_buffer!(reader, &mut machine_tag);
        let load = read_le_word!(reader);
        let start = read_le_word!(reader);
        Ok(Some(Self::Module {
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
        let total_cycles: u64 = read_le_quad_word!(reader);
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

    fn try_read_system<R: Read + Seek>(reader: &mut R) -> Result<Option<Self>> {
        let mut machine_tag = MachineTag::default();
        fill_buffer!(reader, &mut machine_tag);
        let load = read_le_word!(reader);
        Ok(Some(Self::System { machine_tag, load }))
    }
}
