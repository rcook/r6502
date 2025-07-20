use crate::gui::{BitmapFont, Glyph, Padding};
use crate::util::Bits;
use anyhow::{Result, bail};
use bit_vec::BitVec;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::iter::repeat_n;
use std::path::Path;

const MAGIC_HEADER: &[u8] = b"R6502FONT";

#[derive(Debug, PartialEq)]
pub struct R6502BitmapFont {
    char_count: usize,
    char_width: usize,
    char_height: usize,
    glyphs: HashMap<char, BitVec>,
}

impl R6502BitmapFont {
    pub const START_CHAR: u8 = b' ';

    pub fn acorn_mos_1_20() -> Result<Self> {
        R6502BitmapFont::from_bytes(
            include_bytes!("acorn-mos-1.20.font"),
            &Padding {
                left: 0,
                right: 0,
                top: 1,
                bottom: 1,
            },
        )
    }

    pub fn mullard_saa5050() -> Result<Self> {
        R6502BitmapFont::from_bytes(include_bytes!("mullard-saa5050.font"), &Padding::default())
    }

    #[allow(unused)]
    pub fn load(path: &Path, padding: &Padding) -> Result<Self> {
        let f = File::open(path)?;
        Self::from_reader(f, padding)
    }

    #[allow(unused)]
    pub fn from_bytes(bytes: &[u8], padding: &Padding) -> Result<Self> {
        let cursor = Cursor::new(bytes);
        Self::from_reader(cursor, padding)
    }

    pub fn from_reader<R: Read>(mut reader: R, padding: &Padding) -> Result<Self> {
        let mut bytes = [0u8; MAGIC_HEADER.len()];
        reader.read_exact(&mut bytes)?;
        if bytes != MAGIC_HEADER {
            bail!("invalid font")
        }

        reader.read_exact(&mut bytes[0..3])?;

        let char_count = usize::from(bytes[0]);
        let char_width = usize::from(bytes[1]);
        let char_height = usize::from(bytes[2]);
        let char_row_bytes = char_width / 8 + usize::from(char_width % 8 != 0);

        let mut bytes = vec![0u8; char_row_bytes * char_height + 1];
        let glyphs = (0..char_count)
            .map(|i| {
                reader.read_exact(&mut bytes)?;

                // Currently we don't support noncontiguous fonts
                let value = bytes[0];
                if usize::from(value) != i + usize::from(Self::START_CHAR) {
                    bail!("contiguous bitmap fonts only!")
                }

                let c = value as char;
                Ok((
                    c,
                    Self::explode_char(
                        &bytes[1..],
                        char_width,
                        char_height,
                        char_row_bytes,
                        padding,
                    ),
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;
        Ok(Self {
            char_count,
            char_width: padding.left + char_width + padding.right,
            char_height: padding.top + char_height + padding.bottom,
            glyphs,
        })
    }

    fn explode_char(
        bytes: &[u8],
        char_width: usize,
        char_height: usize,
        char_row_bytes: usize,
        padding: &Padding,
    ) -> BitVec {
        let mut glyph = BitVec::new();

        glyph.extend(repeat_n(false, padding.top * char_width));

        let skip_bits = char_row_bytes * 8 - char_width;
        for i in 0..char_height {
            let start = i * char_row_bytes;
            let slice = &bytes[start..start + char_row_bytes];
            glyph.extend(repeat_n(false, padding.left));
            glyph.extend(slice.bits_l2r().skip(skip_bits).take(char_width));
            glyph.extend(repeat_n(false, padding.right));
        }

        glyph.extend(repeat_n(false, padding.bottom * char_width));

        glyph
    }
}

impl BitmapFont for R6502BitmapFont {
    fn char_count(&self) -> usize {
        self.char_count
    }

    fn char_width(&self) -> usize {
        self.char_width
    }

    fn char_height(&self) -> usize {
        self.char_height
    }

    fn get_glyph(&self, c: char) -> Option<Glyph> {
        self.glyphs.get(&c)
    }
}
