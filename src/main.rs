use std::{f32, string};
use std::io::{self, Write, BufWriter};

use glam::{Vec3, Vec4};

use std::env::args;

use crate::graphics::triangle::{RasterTriangle, RasterVertex, Color};
use crate::graphics::rasterizer::Rasterizer;

mod graphics;
mod physics;

fn main() {
    let args: Vec<String> = args().collect();
    let pix_count = args[1].parse::<usize>().expect("failed to parse");

    let White: Color = Color::new(255, 255, 255);
    let Grey: Color = Color::new(128, 128, 128);

    let a1: RasterVertex = RasterVertex::new(Vec4::new(-1.0, 0.0, 0.0, 1.0), White);
    let b1: RasterVertex = RasterVertex::new(Vec4::new(-0.5, 1.0, 0.0, 1.0), White);
    let c1: RasterVertex = RasterVertex::new(Vec4::new(1.0, -1.0, 0.0, 1.0), White);
    let triangle1: RasterTriangle = RasterTriangle::new(a1, b1, c1);

    let a2: RasterVertex = RasterVertex::new(Vec4::new(1.0, 0.0, 1.0, 1.0), Grey);
    let b2: RasterVertex = RasterVertex::new(Vec4::new(0.5, 1.0, 1.0, 1.0), Grey);
    let c2: RasterVertex = RasterVertex::new(Vec4::new(-1.0, -1.0, 1.0, 1.0), Grey);
    let triangle2: RasterTriangle = RasterTriangle::new(a2, b2, c2);

    let mut rasterizer: Rasterizer = Rasterizer::new(pix_count, pix_count);

    rasterizer.clear();
    rasterizer.render_triangle(triangle1);
    rasterizer.render_triangle(triangle2);

    // --- TEST RENDER LOOP ---
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let ramp = " .:-=+*#%@";

    for y in (0..rasterizer.height).step_by(2) {
        for x in 0..rasterizer.width {
            let top_pixel = rasterizer.frame_buff[y * rasterizer.width + x];
            let bot_pixel = rasterizer.frame_buff[(y + 1) * rasterizer.width + x];

            // 1. Calculate Luminance for grayscale (Standard weights)
            let l_top = 0.2126 * (top_pixel.r as f32) + 0.7152 * (top_pixel.g as f32) + 0.0722 * (top_pixel.b as f32);
            let l_bot = 0.2126 * (bot_pixel.r as f32) + 0.7152 * (bot_pixel.g as f32) + 0.0722 * (bot_pixel.b as f32);

            // 2. Average the two virtual pixels (SSAA resolve)
            let avg = (l_top + l_bot) / 2.0 / 255.0;

            // 3. Map to character
            let char_idx = (avg * (ramp.len() - 1) as f32) as usize;
            let c = ramp.chars().nth(char_idx).unwrap_or(' ');

            write!(writer, "{}", c).unwrap();
        }
        write!(writer, "\n").unwrap();
    }
    writer.flush().unwrap();
}

