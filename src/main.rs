mod graphics;
mod physics;

use crate::graphics::triangle::{Color, RasterVertex, RasterTriangle};
use crate::graphics::printer::{Printer, PrinterType};
use crate::graphics::rasterizer::Rasterizer;

use std::io::{stdout, Write};
use glam::Vec3;
use crossterm::terminal;

fn main() {
    test_printer_modes();
}

fn test_printer_modes() {
    let (width_u16, height_u16) = terminal::size().unwrap();
    let (width, height) = (width_u16 as usize, height_u16 as usize * 2);
    
    // 1. Initialize Rasterizer and Printer
    let mut rasterizer = Rasterizer::new(width, height);
    
    // 2. Define two overlapping 2D triangles in NDC space (Z is depth)
    // Red Triangle (further back, Z = 0.5)
    let tri_red = RasterTriangle::new(
        RasterVertex { pos: Vec3::new(-0.8, -0.8, 0.5), rgb: Color::RED, inv_w: 1.0 },
        RasterVertex { pos: Vec3::new(0.8, -0.8, 0.5),  rgb: Color::BLUE, inv_w: 1.0 },
        RasterVertex { pos: Vec3::new(0.0, 0.8, 0.5),   rgb: Color::GREEN, inv_w: 1.0 },
    );



    // 3. Render triangles into the Rasterizer's frame_buff
    rasterizer.clear();
    rasterizer.render_triangle(tri_red);

    // --- TEST 1: ANSI COLOR MODE ---
    let mut color_printer = Printer::new(PrinterType::Color, width, height);
    println!("--- Testing ANSI Color Mode (Press Enter) ---");
    let mut _pause = String::new();
    std::io::stdin().read_line(&mut _pause).unwrap();
    
    color_printer.print(&mut rasterizer.frame_buff);
    stdout().write_all(&color_printer.buff).unwrap();
    std::io::stdin().read_line(&mut _pause).unwrap();
    stdout().write_all(b"\x1b[0m").unwrap(); // Reset terminal colors

    // --- TEST 2: ASCII RAMP MODE ---
    let mut ascii_printer = Printer::new(PrinterType::Ascii, width, height);
    ascii_printer.print(&mut rasterizer.frame_buff);
    stdout().write_all(&ascii_printer.buff).unwrap();
}
