use glam::{UVec2, Vec3};

use crate::graphics::triangle::{Color, RasterTriangle};

struct Rasterizer {
    pub width: usize,
    pub height: usize,
    pub frame_buff: Vec<Color>,
    pub depth_buff: Vec<f32>,
    pub triangles:  Vec<RasterTriangle>,
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            frame_buff: vec![Color {r: 0, g: 0, b: 0}; width * height],
            depth_buff: vec![f32::INFINITY; width * height],
            triangles:  vec![],
        }
    }

    pub fn resize(&mut self, x: u32, y: u32) {
        self.frame_buff.resize((x * y) as usize, Color::new(0, 0, 0));
        self.depth_buff.resize((x * y) as usize, f32::INFINITY);
    }

    pub fn clear(&mut self) {
        self.frame_buff.fill(Color::new(0, 0, 0));
        self.depth_buff.fill(f32::INFINITY);
    }

    pub fn ndc_to_screen(self, ndc: Vec3) -> (usize, usize) {
        let x = ((ndc.x + 1.0) * 0.5 * (self.width as f32 - 1.0)).round() as usize;
        let y = ((1.0 - ndc.y) * 0.5 * (self.height as f32 - 1.0)).round() as usize;
        (x, y)
    }

    pub fn screen_to_ndc(self, screen_xy: (usize, usize)) -> Vec2 {

    }
}
