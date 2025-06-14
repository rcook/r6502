use crate::emulator::util::make_word;
use crate::emulator::{
    AddressRange, ImageFormat, ImageHeader, ImageSlice, MachineTag, DEFAULT_LOAD, DEFAULT_SP,
    DEFAULT_START, R6502_MAGIC_NUMBER, SIM6502_MAGIC_NUMBER,
};
use anyhow::{bail, Error, Result};
use std::fs::File;
use std::io::{Cursor, ErrorKind, Read, Seek};
use std::path::Path;
use std::str::FromStr;

pub struct Image {
    pub format: ImageFormat,
    pub tag: Option<MachineTag>,
    pub load: u16,
    pub start: u16,
    pub sp: u8,
    pub bytes: Vec<u8>,
}

impl Image {
    pub fn load(
        path: &Path,
        default_load: Option<u16>,
        default_start: Option<u16>,
        default_sp: Option<u8>,
    ) -> Result<Self> {
        match File::open(path) {
            Ok(f) => Self::read(f, default_load, default_start, default_sp),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                bail!("Could not find file {}", path.display())
            }
            Err(e) => bail!(e),
        }
    }

    pub fn from_bytes(
        bytes: &[u8],
        default_load: Option<u16>,
        default_start: Option<u16>,
        default_sp: Option<u8>,
    ) -> Result<Self> {
        Self::read(Cursor::new(bytes), default_load, default_start, default_sp)
    }

    #[must_use]
    pub fn slice(&self, range: &AddressRange) -> ImageSlice {
        let image_start = self.load as usize;
        let image_end = image_start + self.bytes.len();
        let range_start = range.start() as usize;
        let range_end = range.end() as usize + 1;

        if range_end <= image_start || range_start >= image_end {
            return ImageSlice {
                bytes: &[],
                load: 0,
            };
        }

        let effective_start = range_start.max(image_start);
        let effective_end = range_end.min(image_end);

        let bytes_start = effective_start - image_start;
        let bytes_end = effective_end - image_start;

        let load = if range_start < image_start {
            (image_start - range_start) as u16
        } else {
            0
        };

        ImageSlice {
            bytes: &self.bytes[bytes_start..bytes_end],
            load,
        }
    }

    fn read<R: Read + Seek>(
        mut reader: R,
        default_load: Option<u16>,
        default_start: Option<u16>,
        default_sp: Option<u8>,
    ) -> Result<Self> {
        let header = Self::read_header(&mut reader, default_load, default_start, default_sp)?;
        let mut values = Vec::new();
        reader.read_to_end(&mut values)?;
        Ok(Self {
            format: header.format,
            tag: header.tag,
            load: header.load,
            start: header.start,
            sp: header.sp,
            bytes: values,
        })
    }

    fn read_header<R: Read + Seek>(
        reader: &mut R,
        default_load: Option<u16>,
        default_start: Option<u16>,
        default_sp: Option<u8>,
    ) -> Result<ImageHeader> {
        let header = Self::read_r6502_header(reader)?;
        if let Some(header) = header {
            return Ok(header);
        }

        let header = Self::read_sim6502_header(reader)?;
        if let Some(header) = header {
            return Ok(header);
        }

        Ok(ImageHeader {
            format: ImageFormat::Raw,
            tag: None,
            load: default_load.unwrap_or(DEFAULT_LOAD),
            start: default_start.unwrap_or(DEFAULT_START),
            sp: default_sp.unwrap_or(DEFAULT_SP),
        })
    }

    fn read_r6502_header<R: Read + Seek>(reader: &mut R) -> Result<Option<ImageHeader>> {
        let mut header = [0x00u8; 10];
        match reader.read_exact(&mut header) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                reader.rewind()?;
                return Ok(None);
            }
            Err(e) => bail!(e),
        }

        let magic_number = make_word(header[1], header[0]);
        if magic_number != R6502_MAGIC_NUMBER {
            reader.rewind()?;
            return Ok(None);
        }

        let mut tag = [0x00; 4];
        tag.copy_from_slice(&header[2..6]);

        let load = make_word(header[7], header[6]);
        let start = make_word(header[9], header[8]);
        Ok(Some(ImageHeader {
            format: ImageFormat::R6502,
            tag: Some(tag),
            load,
            start,
            sp: DEFAULT_SP,
        }))
    }

    fn read_sim6502_header<R: Read + Seek>(reader: &mut R) -> Result<Option<ImageHeader>> {
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

        Ok(Some(ImageHeader {
            format: ImageFormat::Sim65,
            tag: None,
            load,
            start,
            sp,
        }))
    }
}

impl FromStr for Image {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn read_line(values: &mut Vec<u8>, line: &str) -> Result<(u16, u16)> {
            let mut i = line.split_whitespace();

            let Some(s) = i.next() else {
                bail!("input line is empty");
            };

            if s.len() != 4 {
                bail!("invalid address {s}")
            }

            let Ok(addr) = u16::from_str_radix(s, 16) else {
                bail!("invalid address {s}")
            };

            let mut count = 0;
            for s in i {
                if s.len() != 2 {
                    break;
                }

                let Ok(value) = u8::from_str_radix(s, 16) else {
                    break;
                };

                values.push(value);
                count += 1;
            }

            Ok((addr, count))
        }

        let mut values = Vec::new();
        let mut i = s.lines();
        let Some(line) = i.next() else {
            return Ok(Self {
                format: ImageFormat::Listing,
                tag: None,
                load: DEFAULT_LOAD,
                start: DEFAULT_START,
                sp: DEFAULT_SP,
                bytes: values,
            });
        };

        let (mut pc, count) = read_line(&mut values, line)?;

        let load = pc;

        let (temp_pc, overflowed) = pc.overflowing_add(count);
        if overflowed {
            bail!("too much data")
        }

        pc = temp_pc;

        for line in i {
            let (addr, count) = read_line(&mut values, line)?;
            if addr != pc {
                bail!("invalid assembly listing")
            }

            let (temp_pc, overflowed) = pc.overflowing_add(count);
            if overflowed {
                bail!("too much data")
            }

            pc = temp_pc;
        }

        let start = load;
        Ok(Self {
            format: ImageFormat::Listing,
            tag: None,
            load,
            start,
            sp: DEFAULT_SP,
            bytes: values,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::{AddressRange, Image, ImageFormat};
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let input = r" 0E00  A2 00     LDX  #$00
 0E02  BD 0E 0E  LDA  $0E0E, X
 0E05  F0 06     BEQ  $0E0D
 0E07  20 EE FF  JSR  $FFEE
 0E0A  E8        INX
 0E0B  D0 F5     BNE  $0E02
 0E0D  60        RTS
 0E0E  48 45 4C 4C 4F 2C 20 57 4F 52 4C 44 21 00        |HELLO, WORLD!.  |
";
        let image = input.parse::<Image>()?;
        assert_eq!(0x0e00, image.load);
        assert_eq!(28, image.bytes.len());
        Ok(())
    }

    #[test]
    fn r6502() -> Result<()> {
        let bytes = [
            0x65, 0x02, 0x41, 0x43, 0x52, 0x4e, 0x00, 0x0e, 0x00, 0x0e, 0xa2, 0x00, 0xbd, 0x10,
            0x0e, 0xf0, 0x06, 0x20, 0xee, 0xff, 0xe8, 0xd0, 0xf5, 0x4c, 0xc0, 0xff, 0x48, 0x45,
            0x4c, 0x4c, 0x4f, 0x2c, 0x20, 0x57, 0x4f, 0x52, 0x4c, 0x44, 0x21, 0x00,
        ];
        let image = Image::from_bytes(&bytes, None, None, None)?;
        assert_eq!(0x0e00, image.load);
        assert_eq!(0x0e00, image.start);
        assert_eq!(0xff, image.sp);
        assert_eq!(30, image.bytes.len());
        Ok(())
    }

    #[test]
    fn sim6502() -> Result<()> {
        let bytes = [
            0x73, 0x69, 0x6d, 0x36, 0x35, 0x02, 0x00, 0xff, 0x00, 0x10, 0x00, 0x10, 0x20, 0x38,
            0x10, 0xad, 0x6c, 0x10, 0xc9, 0xd2, 0xf0, 0x05, 0xa9, 0x01, 0x4c, 0xf9, 0xff, 0xad,
            0x6d, 0x10, 0xc9, 0x01, 0xf0, 0x05, 0xa9, 0x02, 0x4c, 0xf9, 0xff, 0xad, 0x70, 0x10,
            0xc9, 0x01, 0xf0, 0x05, 0xa9, 0x03, 0x4c, 0xf9, 0xff, 0xad, 0x71, 0x10, 0xc9, 0x00,
            0xf0, 0x05, 0xa9, 0x04, 0x4c, 0xf9, 0xff, 0xa9, 0x00, 0x4c, 0xf9, 0xff, 0xa9, 0x00,
            0x8d, 0x70, 0x10, 0x8d, 0x71, 0x10, 0xa2, 0x10, 0x0e, 0x6c, 0x10, 0x2e, 0x6d, 0x10,
            0x2e, 0x70, 0x10, 0x2e, 0x71, 0x10, 0xad, 0x70, 0x10, 0x38, 0xed, 0x6e, 0x10, 0xa8,
            0xad, 0x71, 0x10, 0xed, 0x6f, 0x10, 0x90, 0x09, 0x8d, 0x71, 0x10, 0x8c, 0x70, 0x10,
            0xee, 0x6c, 0x10, 0xca, 0xd0, 0xd8, 0x60, 0x19, 0x35, 0x12, 0x0a, 0x00, 0xff, 0xff,
        ];
        let image = Image::from_bytes(&bytes, None, None, None)?;
        assert_eq!(0x1000, image.load);
        assert_eq!(0x1000, image.start);
        assert_eq!(0xff, image.sp);
        assert_eq!(114, image.bytes.len());
        Ok(())
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn slice() {
        // 0         10        20
        // 01234567890123456789012345678
        // ----AAA--BBB--CCC---DDD--EE--
        // -----1234567890123456--------
        let a = AddressRange::new(4, 6).expect("Must be valid");
        let b = AddressRange::new(9, 11).expect("Must be valid");
        let c = AddressRange::new(14, 16).expect("Must be valid");
        let d = AddressRange::new(20, 22).expect("Must be valid");
        let e = AddressRange::new(25, 26).expect("Must be valid");
        let image = Image {
            format: ImageFormat::Raw,
            tag: None,
            load: 0x0005,
            start: 0x0000,
            sp: 0xff,
            bytes: (1..=16).collect(),
        };

        let slice = image.slice(&a);
        assert_eq!(vec![1, 2], slice.bytes);
        assert_eq!(1, slice.load);

        let slice = image.slice(&b);
        assert_eq!(vec![5, 6, 7], slice.bytes);
        assert_eq!(0, slice.load);

        let slice = image.slice(&c);
        assert_eq!(vec![10, 11, 12], slice.bytes);
        assert_eq!(0, slice.load);

        let slice = image.slice(&d);
        assert_eq!(vec![16], slice.bytes);
        assert_eq!(0, slice.load);

        let slice = image.slice(&e);
        assert_eq!(Vec::<u8>::new(), slice.bytes);
        assert_eq!(0, slice.load);
    }
}
