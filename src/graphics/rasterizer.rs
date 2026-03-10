use glam::{Vec4};

use crate::graphics::triangle::Color;

struct Rasterizer {
    frame_buff: Vec<Color>,
    depth_buff: Vec<f32>,
}

impl Rasterizer {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            frame_buff: vec![Color {r: 0, g: 0, b: 0}; x * y],
            depth_buff: vec![f32::INFINITY; x * y],
        }
    }
}
