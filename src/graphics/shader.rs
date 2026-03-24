use glam::Vec3;
use crate::{Triangle, graphics::projection::Camera};

struct PointLightSource {
    pub pos: Vec3,
    pub intensity: f32, // intensity for specular and diffuse terms
    pub ambient_intensity: f32, // intensity for ambient terms
}

impl PointLightSource {
    pub fn new(pos: Vec3, intensity: f32, ambient_intensity: f32) -> Self {
        Self {
            pos,
            intensity,
            ambient_intensity,
        }
    }

    pub fn reflect(mut triangle: &Triangle, cam: Camera) {
        
    }
}
