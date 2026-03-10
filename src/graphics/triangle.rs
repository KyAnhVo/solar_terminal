use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};

/// Represents the triangle abc
#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,

    pub normal: Vec4,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex) -> Self {
        let edge1: Vec3 = b.pos.xyz() - a.pos.xyz();
        let edge2: Vec3 = c.pos.xyz() - a.pos.xyz();
        let normal: Vec4 = edge1.cross(edge2).normalize().extend(0.0);
        Self { a, b, c, normal }
    }
}

/// represents the color rgb (no alpha here)
#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b, } }
}

/// represents vertices in world space
#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Vec4,
    pub rgb: Color,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, rgb: Color) -> Self {
        Self {
            pos: Vec4::new(x, y, z, 1.0f32),
            rgb,
        }
    }

    pub fn project(self, m_project: Mat4) -> RasterVertex {
        // note: m_project is m_perspective * m_view
        let pos: Vec4 = m_project * self.pos;
        RasterVertex::new(pos, self.rgb)
    }
}

/// Represents projected vertex, used for
/// perspective correct interpolation
#[derive(Clone, Copy)]
pub struct RasterVertex {
    pub pos: Vec3,
    pub rgb: Color,
    pub inv_w: f32,
}

impl RasterVertex {
    pub fn new(pos: Vec4, rgb: Color) -> Self {
        Self {
            pos: pos.xyz() / pos.w,
            rgb,
            inv_w: 1.0 / pos.w,
        }
    }
}

/// Represents transformed triangle
#[derive(Clone, Copy)]
pub struct RasterTriangle {
    pub a: RasterVertex,
    pub b: RasterVertex,
    pub c: RasterVertex,
    
    pub normal: Vec3,
}

impl RasterTriangle {
    pub fn new(a: RasterVertex, b: RasterVertex, c: RasterVertex) -> Self {
        let ab: Vec3 = b.pos - a.pos;
        let ac: Vec3 = c.pos - a.pos;
        let normal: Vec3 = ab.cross(ac).normalize();

        Self { a, b, c, normal }
    }

    pub fn is_inside(self, p: Vec2) -> bool {
        let (a, b, c): (Vec2, Vec2, Vec2) = (self.a.pos.truncate(), self.b.pos.truncate(), self.c.pos.truncate());
        let (ab, bc, ca): (Vec2, Vec2, Vec2) = (b - a, c - b, a - c);
        let (ap, bp, cp): (Vec2, Vec2, Vec2) = (p - a, p - b, p - c);
        let (s1, s2, s3): (bool, bool, bool) = (ab.perp_dot(ap) >= 0.0, bc.perp_dot(bp) >= 0.0, ca.perp_dot(cp) >= 0.0);

        (s1 && s2 && s3) || !(s1 || s2 || s3)
    }

    pub fn barycentric(self, p: Vec2) -> (f32, f32, f32) {
        let (a, b, c): (Vec2, Vec2, Vec2) = (self.a.pos.truncate(), self.b.pos.truncate(), self.c.pos.truncate());
        let total: f32 = (b - a).perp_dot(c - a);

        (
            (b - p).perp_dot(c - p) / total,
            (c - p).perp_dot(a - p) / total,
            (a - p).perp_dot(b - p) / total,
        )
    }

    pub fn interpolate(self, p: Vec2, av: f32, bv: f32, cv: f32) -> f32 {
        let (alpha, beta, gamma): (f32, f32, f32) = self.barycentric(p);

        let numerator: f32 =
            alpha * av * self.a.inv_w +
            beta  * bv * self.b.inv_w +
            gamma * cv * self.c.inv_w;

        let denominator: f32 = 
            alpha * self.a.inv_w +
            beta  * self.b.inv_w +
            gamma * self.c.inv_w;

        numerator / denominator
    }
}
