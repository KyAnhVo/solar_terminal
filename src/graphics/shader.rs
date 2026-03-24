use glam::{Vec3, Vec4Swizzles};
use crate::{Triangle, graphics::projection::Camera};

pub struct PointLightSource {
    pub pos: Vec3,
    pub diffuse_intensity: Vec3,    // intensity of diffuse term
    pub specular_intensity: Vec3,   // intensity for specular term
    pub ambient_intensity: Vec3,    // intensity for ambient terms
}

impl PointLightSource {
    pub fn new(pos: Vec3, diffuse_intensity: Vec3, specular_intensity: Vec3, ambient_intensity: Vec3) -> Self {
        Self {
            pos,
            diffuse_intensity,
            specular_intensity,
            ambient_intensity,
        }
    }

    pub fn reflect(self, triangle: Triangle, cam: Camera) -> Vec3 {
        // uses triangle.a as ref for most things, assume all 3 are equivalent

        let kd: Vec3 = triangle.a.rgb.to_vec3();
        let ks: Vec3 = triangle.material.ks;
        let ka: Vec3 = triangle.material.ka;
        let p: f32 = triangle.material.p;

        let n: Vec3 = triangle.normal.xyz();
        let v: Vec3 = cam.e.xyz() - triangle.a.pos.xyz();
        let l: Vec3 = self.pos - triangle.a.pos.xyz();
        let h_prenormalize: Vec3 = v + l;
        let r2: f32 = h_prenormalize.length_squared();
        let h = h_prenormalize.normalize();

        let ia: Vec3 = self.ambient_intensity;
        let id: Vec3 = self.diffuse_intensity;
        let is: Vec3 = self.specular_intensity;
        let ambient_term: Vec3 = ka * ia;
        let diffuse_term: Vec3 = kd * (id / r2) * (n.dot(l)).max(0.0);
        let specular_term: Vec3 = ks * (is / r2) * (n.dot(h)).max(0.0).powf(p);
        
        ambient_term + diffuse_term + specular_term
    }
}
