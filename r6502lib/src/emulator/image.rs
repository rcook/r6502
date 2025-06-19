use crate::emulator::r6502_image::Image as R6502Image;
use crate::emulator::{AddressRange, Cpu, ImageSlice, MachineTag, OtherImage};
use anyhow::{bail, Error, Result};
use std::fs::File;
use std::io::{Cursor, ErrorKind, Read, Seek};
use std::path::Path;
use std::str::FromStr;

pub enum Image {
    R6502(R6502Image),
    Other(OtherImage),
}

impl Image {
    pub fn from_file(path: &Path) -> Result<Self> {
        match File::open(path) {
            Ok(f) => Self::from_reader(f),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                bail!("Could not find image file {path}", path = path.display())
            }
            Err(e) => bail!(e),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Self::from_reader(Cursor::new(bytes))
    }

    #[must_use]
    pub const fn machine_tag(&self) -> Option<MachineTag> {
        match self {
            Self::R6502(image) => Some(image.machine_tag()),
            Self::Other(_) => None,
        }
    }

    #[must_use]
    pub const fn load(&self) -> Option<u16> {
        match self {
            Self::R6502(image) => image.load(),
            Self::Other(other) => other.load(),
        }
    }

    #[must_use]
    pub const fn start(&self) -> Option<u16> {
        match self {
            Self::R6502(image) => Some(image.start()),
            Self::Other(other) => other.start(),
        }
    }

    #[must_use]
    pub const fn sp(&self) -> Option<u8> {
        match self {
            Self::R6502(image) => image.sp(),
            Self::Other(other) => other.sp(),
        }
    }
    #[must_use]
    pub const fn bytes(&self) -> &Vec<u8> {
        match self {
            Self::R6502(image) => image.bytes(),
            Self::Other(other) => other.bytes(),
        }
    }

    pub const fn set_initial_cpu_state(&self, cpu: &mut Cpu) {
        match self {
            Self::R6502(image) => image.set_initial_cpu_state(cpu),
            Self::Other(image) => image.set_initial_cpu_state(cpu),
        }
    }

    #[must_use]
    pub fn slice(&self, range: &AddressRange) -> ImageSlice {
        let image_start = self.load().unwrap_or_default() as usize;
        let bytes = self.bytes();
        let image_end = image_start + bytes.len();
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
            bytes: &bytes[bytes_start..bytes_end],
            load,
        }
    }

    fn from_reader<R: Read + Seek>(mut reader: R) -> Result<Self> {
        if let Some(image) = R6502Image::try_from_reader(&mut reader)? {
            return Ok(Self::R6502(image));
        }
        let image = OtherImage::from_reader(&mut reader)?;
        Ok(Self::Other(image))
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

        let mut bytes = Vec::new();
        let mut i = s.lines();
        let Some(line) = i.next() else {
            return Ok(Self::Other(OtherImage::new_raw(bytes)));
        };

        let (mut pc, count) = read_line(&mut bytes, line)?;

        let load = pc;

        let (temp_pc, overflowed) = pc.overflowing_add(count);
        if overflowed {
            bail!("too much data")
        }

        pc = temp_pc;

        for line in i {
            let (addr, count) = read_line(&mut bytes, line)?;
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
        Ok(Self::Other(OtherImage::new_listing(load, start, bytes)))
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::{AddressRange, Image, OtherImage, DEFAULT_SP};
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

        let load = image.load().expect("Must be set");
        assert_eq!(0x0e00, load);
        assert_eq!(28, image.bytes().len());
        Ok(())
    }

    #[test]
    fn r6502() -> Result<()> {
        let bytes = [
            0x65, 0x02, 0x00, 0x41, 0x43, 0x52, 0x4e, 0x00, 0x0e, 0x00, 0x0e, 0xa2, 0x00, 0xbd,
            0x10, 0x0e, 0xf0, 0x06, 0x20, 0xee, 0xff, 0xe8, 0xd0, 0xf5, 0x4c, 0xc0, 0xff, 0x48,
            0x45, 0x4c, 0x4c, 0x4f, 0x2c, 0x20, 0x57, 0x4f, 0x52, 0x4c, 0x44, 0x21, 0x00,
        ];
        let image = Image::from_bytes(&bytes)?;
        assert_eq!(0x0e00, image.load().unwrap_or_default());
        assert_eq!(0x0e00, image.start().unwrap_or_default());
        assert_eq!(0xff, image.sp().unwrap_or(DEFAULT_SP));
        assert_eq!(30, image.bytes().len());
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
        let image = Image::from_bytes(&bytes)?;
        assert_eq!(0x1000, image.load().unwrap_or_default());
        assert_eq!(0x1000, image.start().unwrap_or_default());
        assert_eq!(0xff, image.sp().unwrap_or(DEFAULT_SP));
        assert_eq!(114, image.bytes().len());
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
        let image = Image::Other(OtherImage::new_sim6502(
            0x0005,
            0x0000,
            0xff,
            (1..=16).collect(),
        ));

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
