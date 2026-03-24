use glam::{Mat4};

use crate::Triangle;
use crate::RasterTriangle;
use crate::Rasterizer;
use crate::Camera;
use crate::graphics::shader::PointLightSource;
use crate::Printer;
use crate::PrinterType;
use crossterm::terminal;

pub struct Pipeline {
    rasterizer: Rasterizer,
    cam: Camera,
    is_perspective: bool,
    light_sources: Vec<PointLightSource>,
    printer: Printer,
    printer_type: PrinterType,
    screen_height: u16,
    screen_width: u16,
}

impl Pipeline {
    fn new(
        cam: Camera, is_perspective: bool,
        printer_type: PrinterType,
    ) -> Self {
        let (screen_width, screen_height): (u16, u16) = terminal::size().unwrap();
        let (real_width, real_height): (usize, usize) = (screen_width as usize, screen_height as usize * 2);
        let rasterizer: Rasterizer = Rasterizer::new(real_width, real_height);
        let printer: Printer = Printer::new(printer_type, real_width, real_height);
        Self {
            rasterizer,
            cam,
            is_perspective,
            light_sources: vec![],
            printer,
            printer_type,
            screen_height,
            screen_width
        }
    }

    fn resize(&mut self) {
        let (screen_width, screen_height): (u16, u16) = terminal::size().unwrap();
        if self.screen_width == screen_width && self.screen_height == screen_height {
            return;
        }
        let (real_width, real_height): (usize, usize) = (screen_width as usize, screen_height as usize * 2);
        self.rasterizer.resize(real_width, real_height);
        self.printer.resize(real_width, real_height);
    }

    fn render(&mut self, triangles: Vec<Triangle>, n: f32, f: f32) {
        self.resize();
        let m_view: Mat4 = self.cam.m_view();
        let m_persp: Mat4 = if self.is_perspective {
            self.cam.m_perspective(n, f) * m_view
        } else {
            self.cam.m_ortho(n, f) * m_view
        };
        let mut raster_triangles: Vec<RasterTriangle> = vec![];
        for triangle in triangles.iter() {
            raster_triangles.push(RasterTriangle::from_world_view(*triangle, m_persp));
        }
        for raster_triangle in raster_triangles.iter() {
            self.rasterizer.render_triangle(raster_triangle);
        }
    }
}
