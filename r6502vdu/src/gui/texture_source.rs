use sdl3::pixels::Color;

const PIXEL_BYTES: usize = 4;

pub struct TextureSource {
    pixel_bytes: usize,
    width: usize,
    height: usize,
    draw_colour: Color,
    bytes: Vec<u8>,
}

impl TextureSource {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixel_bytes: PIXEL_BYTES,
            width,
            height,
            draw_colour: Color::WHITE,
            bytes: vec![0; PIXEL_BYTES * width * height],
        }
    }

    #[must_use]
    pub fn pixel_bytes(&self) -> usize {
        self.pixel_bytes
    }

    #[allow(unused)]
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[allow(unused)]
    pub fn set_draw_colour(&mut self, value: Color) {
        self.draw_colour = value;
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize) {
        let idx = PIXEL_BYTES * ((self.height - 1 - y) * self.width + x);
        self.bytes[idx] = self.draw_colour.r;
        self.bytes[idx + 1] = self.draw_colour.g;
        self.bytes[idx + 2] = self.draw_colour.b;
        self.bytes[idx + 3] = self.draw_colour.a;
    }
}
