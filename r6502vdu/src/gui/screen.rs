use crate::font::Font as ArgsFont;
use crate::gui::{
    BdfBitmapFont, BitmapFont, DrawMetrics, R6502BitmapFont, State, TextureSource, WindowState,
};
use crate::{f32, u8, u32};
use anyhow::Result;
use log::warn;
use sdl3::Sdl;
use sdl3::iostream::IOStream;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::{FRect, Texture, TextureAccess, TextureCreator, TextureQuery, WindowCanvas};
use sdl3::ttf::{Font, Sdl3TtfContext};
use sdl3::video::{Window, WindowContext};
use std::cell::RefCell;

const GRAPHICS_WIDTH: f32 = 800.0;
const GRAPHICS_HEIGHT: f32 = 600.0;

/*
const COLOURS: [Color; 8] = [
    Color::WHITE,
    Color::GREY,
    Color::RED,
    Color::GREEN,
    Color::BLUE,
    Color::MAGENTA,
    Color::YELLOW,
    Color::CYAN,
];
*/

pub struct Screen<'a> {
    window: Window,
    canvas: RefCell<WindowCanvas>,
    texture_creator: TextureCreator<WindowContext>,
    texture: Texture,
    status_font: Font<'a>,
    font: Box<dyn BitmapFont>,
    columns: usize,
    rows: usize,
    previous_lines: Vec<String>,
    current_line: String,
    cursor_visible: bool,
    #[allow(clippy::struct_field_names)]
    full_screen: bool,
}

impl<'a> Screen<'a> {
    pub fn new(
        sdl: &Sdl,
        ttf: &'a Sdl3TtfContext,
        columns: usize,
        rows: usize,
        zoom: usize,
        state: Option<&State>,
        font: &ArgsFont,
    ) -> Result<Self> {
        let font: Box<dyn BitmapFont> = match font {
            ArgsFont::Acorn => Box::new(R6502BitmapFont::acorn_mos_1_20()?),
            ArgsFont::Bedstead => Box::new(BdfBitmapFont::bedstead()?),
            ArgsFont::MullardSaa5050 => Box::new(R6502BitmapFont::mullard_saa5050()?),
        };

        let window = Self::create_window(sdl, font.as_ref(), columns, rows, zoom, state)?;
        let canvas = window.clone().into_canvas();
        let texture_creator = canvas.texture_creator();
        let texture = Self::create_texture(font.as_ref(), &texture_creator)?;

        let bytes = include_bytes!("CourierPrime-Regular.ttf");
        let stream = IOStream::from_bytes(bytes)?;
        let status_font = ttf.load_font_from_iostream(stream, 48.0)?;

        Ok(Self {
            window,
            canvas: RefCell::new(canvas),
            texture_creator,
            texture,
            status_font,
            font,
            columns,
            rows,
            previous_lines: Vec::new(),
            current_line: String::new(),
            cursor_visible: false,
            full_screen: false,
        })
    }

    #[must_use]
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn backspace(&mut self) -> Result<()> {
        self.current_line.pop();
        self.scroll();
        self.present()
    }

    pub fn clear_line(&mut self) -> Result<()> {
        self.current_line.clear();
        self.present()
    }

    pub fn clear_screen(&mut self) -> Result<()> {
        self.previous_lines.clear();
        self.current_line.clear();
        self.scroll();
        self.present()
    }

    pub fn new_line(&mut self) -> Result<()> {
        let s = self.current_line.clone();
        let len = s.len();
        if len == 0 {
            self.previous_lines.push(String::new());
        } else {
            let mut i = 0;
            while i < len {
                let end = (i + self.columns).min(len);
                self.previous_lines.push(String::from(&s[i..end]));
                i = end;
            }
        }
        self.current_line.clear();
        self.scroll();
        self.present()
    }

    pub fn update_cursor(&mut self) -> Result<()> {
        self.cursor_visible = !self.cursor_visible;
        self.present()
    }

    pub fn toggle_full_screen(&mut self) -> Result<()> {
        self.full_screen = !self.full_screen;
        self.window.set_fullscreen(self.full_screen)?;
        self.present()
    }

    pub fn write(&mut self, s: &str) -> Result<()> {
        self.current_line.push_str(s);
        self.scroll();
        self.present()
    }

    pub fn show(&self) -> Result<()> {
        self.present()?;
        self.canvas.borrow_mut().window_mut().show();
        Ok(())
    }

    fn create_window(
        sdl: &Sdl,
        font: &dyn BitmapFont,
        columns: usize,
        rows: usize,
        zoom: usize,
        state: Option<&State>,
    ) -> Result<Window> {
        let video = sdl.video()?;

        let (width, height) = if let Some(state) = state {
            (state.window.width, state.window.height)
        } else {
            let width = columns * font.char_width() * zoom;
            let height = rows * font.char_height() * zoom;
            (u32::try_from(width)?, u32::try_from(height)?)
        };

        let mut window_builder = video.window("r6502", width, height);
        window_builder.resizable().hidden().opengl();

        if let Some(state) = state {
            window_builder.position(state.window.x, state.window.y);
            match state.window.state {
                WindowState::FullScreen => _ = window_builder.fullscreen(),
                WindowState::Maximized => _ = window_builder.maximized(),
                WindowState::Minimized => _ = window_builder.minimized(),
                WindowState::Normal => {}
            }
        } else {
            window_builder.position_centered();
        }

        let window = window_builder.build()?;
        video.text_input().start(&window);

        Ok(window)
    }

    fn create_texture(
        font: &dyn BitmapFont,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Result<Texture> {
        let mut texture = texture_creator.create_texture(
            texture_creator.default_pixel_format(),
            TextureAccess::Streaming,
            u32::try_from(font.char_width()).unwrap(),
            u32::try_from(font.char_height() * font.char_count()).unwrap(),
        )?;

        let mut texture_source =
            TextureSource::new(font.char_width(), font.char_height() * font.char_count());
        for i in 0..font.char_count() {
            Self::make_char_texture(
                font,
                &mut texture_source,
                0,
                i,
                (u8!(i)? + R6502BitmapFont::START_CHAR) as char,
            );
        }

        texture.update(
            None,
            texture_source.bytes(),
            texture_source.pixel_bytes() * font.char_width(),
        )?;
        Ok(texture)
    }

    fn make_char_texture(
        font: &dyn BitmapFont,
        texture_source: &mut TextureSource,
        column: usize,
        row: usize,
        c: char,
    ) {
        /*
        let mut rand = rng();
        let colour = COLOURS[rand.random_range(0..COLOURS.len())];
        texture_source.set_draw_colour(colour);
        */
        texture_source.set_draw_colour(Color::WHITE);

        let x = column * font.char_width();
        let y = texture_source.height() - (row + 1) * font.char_height();

        if let Some(glyph) = font.get_glyph(c) {
            for char_row in 0..font.char_height() {
                let y_offset = font.char_height() - char_row - 1;
                for char_column in 0..font.char_width() {
                    let i = char_row * font.char_width() + char_column;
                    if let Some(bit) = glyph.get(i)
                        && bit
                    {
                        texture_source.draw_pixel(x + char_column, y + y_offset);
                    }
                }
            }
        } else {
            warn!("no glyph for character code {code}", code = c as u8);
        }
    }

    fn present(&self) -> Result<()> {
        let metrics = self.calculate_redraw_metrics()?;

        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::GREY);
        canvas.fill_rect(Rect::new(0, 0, metrics.canvas_width, metrics.canvas_height))?;

        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(
            i32::try_from(metrics.left)?,
            i32::try_from(metrics.top)?,
            metrics.screen_width,
            metrics.screen_height,
        ))?;

        let mut row = 0;
        for line in &self.previous_lines {
            assert!(line.len() <= self.columns);
            self.draw_line(&mut canvas, &metrics, row, line)?;
            let len = line.len();
            let rows = (len / self.columns + usize::from(len % self.columns != 0)).max(1);
            row += rows;
        }
        self.draw_line(&mut canvas, &metrics, row, &self.current_line)?;

        let (column, row) = self.draw_cursor(&mut canvas, &metrics, row)?;

        /*
        canvas
            .render_geometry(
                &[
                    metrics.make_vertex(100.0, 100.0, Color::RED)?,
                    metrics.make_vertex(200.0, 200.0, Color::GREEN)?,
                    metrics.make_vertex(500.0, 50.0, Color::BLUE)?,
                ],
                None,
                VertexIndices::Sequential,
            )?;
        */

        // Status info
        self.draw_status(&mut canvas, &metrics, &format!("{column},{row}"))?;

        canvas.present();
        Ok(())
    }

    // TBD: This duplicates logic in "present"!
    fn scroll(&mut self) {
        let mut row = 0;
        for line in &self.previous_lines {
            assert!(line.len() <= self.columns);
            let len = line.len();
            let rows = (len / self.columns + usize::from(len % self.columns != 0)).max(1);
            row += rows;
        }

        let len = self.current_line.len();
        let c_row = row + len / self.columns;
        assert!(c_row <= self.rows);
        if c_row == self.rows {
            _ = self.previous_lines.remove(0);
        }
    }

    fn draw_line(
        &self,
        canvas: &mut WindowCanvas,
        metrics: &DrawMetrics,
        start_row: usize,
        s: &str,
    ) -> Result<()> {
        let start_idx = start_row * self.columns;
        for (i, c) in s.char_indices() {
            let idx = start_idx + i;
            let c_column = idx % self.columns;
            let c_row = idx / self.columns;
            self.draw_char(canvas, metrics, c, c_column, c_row)?;
        }
        Ok(())
    }

    fn draw_cursor(
        &self,
        canvas: &mut WindowCanvas,
        metrics: &DrawMetrics,
        row: usize,
    ) -> Result<(usize, usize)> {
        let len = self.current_line.len();
        let c_column = len % self.columns;
        let c_row = row + len / self.columns;
        self.draw_char(
            canvas,
            metrics,
            if self.cursor_visible { '_' } else { ' ' },
            c_column,
            c_row,
        )?;
        Ok((c_column, c_row))
    }

    fn draw_status(&self, canvas: &mut WindowCanvas, metrics: &DrawMetrics, s: &str) -> Result<()> {
        let surface = self.status_font.render(s).blended(Color::RED)?;
        let texture = self.texture_creator.create_texture_from_surface(&surface)?;
        let TextureQuery { width, height, .. } = texture.query();

        if width <= metrics.canvas_width && height <= metrics.canvas_height {
            let target = FRect::new(
                f32!(metrics.canvas_width - width)?,
                f32!(metrics.canvas_height - height)?,
                f32!(width)?,
                f32!(height)?,
            );
            canvas.copy(&texture, None, Some(target))?;
        }

        Ok(())
    }

    fn draw_char(
        &self,
        canvas: &mut WindowCanvas,
        metrics: &DrawMetrics,
        c: char,
        c_column: usize,
        c_row: usize,
    ) -> Result<()> {
        let sprite_rect = FRect::new(
            0.0,
            f32!((c as usize - R6502BitmapFont::START_CHAR as usize) * self.font.char_height())?,
            f32!(self.font.char_width())?,
            f32!(self.font.char_height())?,
        );
        let canvas_rect = FRect::new(
            f32!(metrics.left as usize + c_column * metrics.char_width)?,
            f32!(metrics.top as usize + c_row * metrics.char_height)?,
            f32!(metrics.char_width)?,
            f32!(metrics.char_height)?,
        );
        canvas.copy(&self.texture, Some(sprite_rect), Some(canvas_rect))?;
        Ok(())
    }

    fn calculate_redraw_metrics(&self) -> Result<DrawMetrics> {
        let (canvas_width, canvas_height) = self.canvas.borrow().output_size()?;
        let pixel_width = self.font.char_width() * self.columns;
        let pixel_height = self.font.char_height() * self.rows;
        let scale_x = f32!(canvas_width)? / f32!(pixel_width)?;
        let scale_y = f32!(canvas_height)? / f32!(pixel_height)?;
        let scale = scale_x.min(scale_y);
        let scaled_width = u32!(scale * f32!(pixel_width)?)?;
        let scaled_height = u32!(scale * f32!(pixel_height)?)?;
        let char_width = scaled_width as usize / self.columns;
        let char_height = scaled_height as usize / self.rows;
        let screen_width = u32::try_from(char_width * self.columns)?;
        let screen_height = u32::try_from(char_height * self.rows)?;
        let left = (canvas_width - screen_width) / 2;
        let top = (canvas_height - screen_height) / 2;
        Ok(DrawMetrics {
            canvas_width,
            canvas_height,
            char_width,
            char_height,
            screen_width,
            screen_height,
            left,
            top,
            graphics_width: GRAPHICS_WIDTH,
            graphics_height: GRAPHICS_HEIGHT,
        })
    }
}
