use anyhow::Result;
use sdl3::rect::Rect;
use sdl3::video::Display;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

pub fn get_default_bounds(display: Display) -> Result<Rect> {
    let bounds = display.get_usable_bounds()?;
    let screen_width = bounds.width();
    let screen_height = bounds.height();
    let width = DEFAULT_WIDTH.min(screen_width);
    let height = DEFAULT_HEIGHT.min(screen_height);
    let x = i32::try_from(screen_width - width)? / 2;
    let y = i32::try_from(screen_height - height)? / 2;
    Ok(Rect::new(x, y, width, height))
}
