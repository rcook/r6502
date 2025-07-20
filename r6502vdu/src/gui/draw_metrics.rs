#![allow(unused)]

use crate::f32;
use anyhow::Result;
use sdl3::pixels::{Color, FColor};
use sdl3::render::{FPoint, Vertex};

pub struct DrawMetrics {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub char_width: usize,
    pub char_height: usize,
    pub screen_width: u32,
    pub screen_height: u32,
    pub left: u32,
    pub top: u32,
    pub graphics_width: f32,
    pub graphics_height: f32,
}

impl DrawMetrics {
    pub fn make_vertex(&self, x: f32, y: f32, colour: FColor) -> Result<Vertex> {
        let point = self.make_point(x, y)?;
        Ok(Vertex {
            position: point,
            color: colour,
            tex_coord: point,
        })
    }

    fn make_point(&self, x: f32, y: f32) -> Result<FPoint> {
        Ok(FPoint::new(self.get_x(x)?, self.get_y(y)?))
    }

    fn get_x(&self, x: f32) -> Result<f32> {
        Ok(x / self.graphics_width * f32!(self.screen_width)? + f32!(self.left)?)
    }

    fn get_y(&self, y: f32) -> Result<f32> {
        Ok(y / self.graphics_height * f32!(self.screen_height)? + f32!(self.top)?)
    }
}
