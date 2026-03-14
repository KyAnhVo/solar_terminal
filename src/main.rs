mod graphics;
mod physics;

use crate::graphics::projection::Camera;
use crate::graphics::triangle::{Color, Triangle, RasterVertex, RasterTriangle};
use crate::graphics::printer::{Printer, PrinterType};
use crate::graphics::rasterizer::Rasterizer;

use crate::physics::cosmic_body::{CosmicBody, CosmicSimulator};

use std::env::args;
use std::f32;
use std::{thread, time};
use std::io::{stdout, Write};
use glam::{Mat3, Mat4, Vec3};
use crossterm::terminal;
use std::time::Instant;

fn main() {
    let (width_u16, height_u16) = terminal::size().unwrap();
    let (width, height) = (width_u16 as usize, height_u16 as usize * 2);

    test_planets(width, height, 2);
}

fn test_printer_modes(width: usize, height: usize) {

    
    let mut rasterizer = Rasterizer::new(width, height);

    let depth: f32 = 1.0;
    let mut triangles: Vec<RasterTriangle> = vec![];
    for i in 0..1000 {
        triangles.push(RasterTriangle::new(
            RasterVertex { pos: Vec3::new(-0.1, -0.1, depth - i as f32 * 0.001), rgb: Color::GREEN, inv_w: 1.0 },
            RasterVertex { pos: Vec3::new(0.1, -0.1, depth - i as f32 * 0.001),  rgb: Color::GREEN, inv_w: 1.0 },
            RasterVertex { pos: Vec3::new(0.0, 0.1, depth - i as f32 * 0.001),   rgb: Color::GREEN, inv_w: 1.0 },
        ));
    }

    println!("finish setup");
    let mut _pause = String::new();

    let start: Instant = Instant::now();

    // 3. Render triangles into the Rasterizer's frame_buff
    rasterizer.clear();
    for triangle in triangles.iter() {
        rasterizer.render_triangle(triangle);
    }

    let end: Instant = Instant::now();
    let dt: f32 = end.duration_since(start).as_secs_f32();

    println!("Time elapsed: {} for {} triangles", dt, triangles.len());
    std::io::stdin().read_line(&mut _pause).unwrap();

    // --- TEST 1: ANSI COLOR MODE ---
    let mut color_printer = Printer::new(PrinterType::Color, width, height);
    color_printer.print(&mut rasterizer.frame_buff);
    stdout().write_all(&color_printer.buff).unwrap();
    std::io::stdin().read_line(&mut _pause).unwrap();
    stdout().write_all(b"\x1b[0m").unwrap(); // Reset terminal colors

    // --- TEST 2: ASCII RAMP MODE ---
    let mut ascii_printer = Printer::new(PrinterType::Ascii, width, height);
    ascii_printer.print(&mut rasterizer.frame_buff);
    stdout().write_all(&ascii_printer.buff).unwrap();
}

fn test_planet(width: usize, height: usize, cam_angle: f32) {
    let print_type: PrinterType = if args().collect::<Vec<String>>()[1] == "-a" {
        PrinterType::Ascii
    } else {
        PrinterType::Color
    };
    let mut rasterizer: Rasterizer = Rasterizer::new(width, height);

    let sphere: CosmicBody = CosmicBody::new(Vec3::new(-7.0, 0.0, 0.0), 0, Vec3::ZERO, Color::WHITE, 3.0);
    let triangles: Vec<Triangle> = sphere.to_triangles();

    let rot: Mat3 = Mat3::from_rotation_y(cam_angle);

    let cam: Camera = Camera::new(
        Vec3::Y.extend(0.0), 
        (rot * Vec3::Y).extend(0.0), 
        (rot * Vec3::Y * 20.0).extend(1.0), 
        f32::consts::PI / 4.0, 
        width as f32 / height as f32
        );
    let projection: Mat4 = cam.m_perspective(0.01, 50.0) * cam.m_view();

    let mut raster_triangles: Vec<RasterTriangle> = vec![];
    for triangle in triangles.iter() {
        raster_triangles.push(RasterTriangle::from_world_view(*triangle, projection));
    }

    for raster_triangle in raster_triangles.iter() {
        rasterizer.render_triangle(raster_triangle);
    }

    let mut color_printer = Printer::new(print_type, width, height);
    color_printer.print(&mut rasterizer.frame_buff);
    stdout().write_all(&color_printer.buff).unwrap();
}

fn test_planets(width: usize, height: usize, day_count_incremental: u32) {
    let print_type: PrinterType = if args().collect::<Vec<String>>()[1] == "-a" {
        PrinterType::Ascii
    } else {
        PrinterType::Color
    };
    let mut rasterizer: Rasterizer = Rasterizer::new(width, height);
    let mut cosmic_simulator = CosmicSimulator::new();

    let cam: Camera = Camera::new(
        (CosmicBody::rot_x(f32::consts::PI / 2.5) * Vec3::Y).extend(0.0), 
        (CosmicBody::rot_x(f32::consts::PI / 2.5) * Vec3::Z).extend(0.0), 
        (CosmicBody::rot_x(f32::consts::PI / 2.5) * Vec3::Z * -1000.0).extend(1.0), 
        f32::consts::PI / 4.0, 
        width as f32 / height as f32
    );
    let projection: Mat4 = cam.m_perspective(0.01, 500.0) * cam.m_view();

    loop {
        cosmic_simulator.orbit(day_count_incremental);
        let triangles: Vec<Triangle> = cosmic_simulator.to_triangles();
        let mut raster_triangles: Vec<RasterTriangle> = vec![];
        for triangle in triangles.iter() {
            raster_triangles.push(RasterTriangle::from_world_view(*triangle, projection));
        }
        for raster_triangle in raster_triangles.iter() {
            rasterizer.render_triangle(raster_triangle);
        }
        let mut color_printer = Printer::new(print_type, width, height);
        color_printer.print(&mut rasterizer.frame_buff);
        stdout().write_all(&color_printer.buff).unwrap();
        rasterizer.clear();
        
        thread::sleep(time::Duration::from_millis(10));
    }

}
