use crate::gui::{BitmapFont, Glyph};
use anyhow::{Result, anyhow};
use bdf2::Font;
use std::io::{Cursor, Read};
use std::path::Path;

pub struct BdfBitmapFont {
    char_count: usize,
    char_width: usize,
    char_height: usize,
    font: Font,
}

impl BdfBitmapFont {
    pub fn bedstead() -> Result<Self> {
        Self::from_bytes(include_bytes!("bedstead-20.bdf"))
    }

    #[allow(unused)]
    pub fn load(path: &Path) -> Result<Self> {
        Self::new(bdf2::open(path)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let cursor = Cursor::new(bytes);
        Self::from_reader(cursor)
    }

    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        Self::new(bdf2::read(reader)?)
    }

    fn new(font: Font) -> Result<Self> {
        let char_count = 95; // TBD

        let glyph = font
            .glyphs()
            .get(&'A')
            .ok_or_else(|| anyhow!("could not get glyph"))?;
        let char_width = glyph.width() as usize;
        let char_height = glyph.height() as usize;

        Ok(Self {
            char_count,
            char_width,
            char_height,
            font,
        })
    }
}

impl BitmapFont for BdfBitmapFont {
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
        let glyph = self.font.glyphs().get(&c)?;
        assert_eq!(self.char_width, glyph.width() as usize);
        assert_eq!(self.char_height, glyph.height() as usize);
        Some(glyph.map().get_ref())
    }
}
