use crate::gui::Glyph;
use anyhow::{Result, anyhow};

pub trait BitmapFont {
    fn char_count(&self) -> usize;

    fn char_width(&self) -> usize;

    fn char_height(&self) -> usize;

    fn get_glyph(&self, c: char) -> Option<Glyph>;

    #[allow(unused)]
    fn dump_char(&self, c: char) -> Result<()> {
        let glyph = self
            .get_glyph(c)
            .ok_or_else(|| anyhow!("character {c} not defined in font"))?;
        let mut i = glyph.iter();
        for _ in 0..self.char_height() {
            for _ in 0..self.char_width() {
                let bit = i.next().unwrap();
                if bit { print!("#") } else { print!("_") }
            }
            println!();
        }
        Ok(())
    }
}
