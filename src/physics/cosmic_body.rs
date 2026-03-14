use std::f32;

use rand::{RngExt};
use glam::{Mat3, Mat4, Vec3, Vec4};

use crate::graphics::triangle::{Color, Triangle, Vertex};

#[derive(Clone, Copy)]
pub struct CosmicBody {
    pub original_pos: Vec3,

    /// position wrt the sun
    pub pos: Vec3,

    /// velocity of rotation around the sun (rad/day)
    pub orbital_angular_velocity: f32,

    /// days per orbit
    pub days_per_orbit: u32,

    /// normal of the plane containing the rotate path
    pub rotational_normal: Vec3,

    /// color of the cosmic body, rgb
    pub color: Color,

    /// radius to draw the cosmic body
    pub radius: f32,
}

impl CosmicBody {
    pub fn rot_x(theta: f32) -> Mat3 {
        let (sin, cos): (f32, f32) = (theta.sin(), theta.cos());
        Mat3::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, cos, sin),
            Vec3::new(0.0, -sin, cos)
        )
    }

    pub fn rot_y(theta: f32) -> Mat3 {
        let (sin, cos): (f32, f32) = (theta.sin(), theta.cos());
        Mat3::from_cols(
            Vec3::new(cos, 0.0, -sin),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(sin, 0.0, cos),
        )
    }

    pub fn rot_z(theta: f32) -> Mat3 {
        let (sin, cos): (f32, f32) = (theta.sin(), theta.cos());
        Mat3::from_cols(
            Vec3::new(cos, sin, 0.0),
            Vec3::new(-sin, cos, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        )
    }

    pub fn new(
        original_pos: Vec3, 
        days_per_orbit: u32,
        rotational_normal: Vec3, 
        color: Color, 
        radius: f32
    ) -> Self {
        let orbital_angular_velocity: f32 = if days_per_orbit == 0 { 
            0.0
        } else { 
            2.0 * f32::consts::PI / (days_per_orbit as f32) 
        };

        Self {
            original_pos,
            pos: original_pos,
            orbital_angular_velocity: orbital_angular_velocity.sqrt() / 10.0,
            days_per_orbit,
            rotational_normal: rotational_normal.normalize(),
            color,
            radius,
        }
    }

    pub fn orbit(&mut self, days_passed: u32) {
        if self.orbital_angular_velocity.abs() < f32::EPSILON {
            return;
        }
        let rad: f32 = self.orbital_angular_velocity * days_passed as f32;
        let r: Mat3 = if self.rotational_normal == Vec3::Z {
            Mat3::from_cols(
                Vec3::new(rad.cos(),    rad.sin(),  0.0),
                Vec3::new(-rad.sin(),   rad.cos(),  0.0),
                Vec3::new(0.0,          0.0,        1.0),
            )
        } else {
            let nnt: Mat3 = Mat3::from_cols(
                self.rotational_normal * self.rotational_normal.x,
                self.rotational_normal * self.rotational_normal.y,
                self.rotational_normal * self.rotational_normal.z
            );
            let n_dual: Mat3 = Mat3::from_cols(
                Vec3::new(0.0,                          self.rotational_normal.z,   -self.rotational_normal.y),
                Vec3::new(-self.rotational_normal.z,    0.0,                        self.rotational_normal.x),
                Vec3::new(self.rotational_normal.y,     -self.rotational_normal.x,  0.0),
            );
            rad.cos() * Mat3::IDENTITY 
            + (1.0 - rad.cos()) * nnt
            + rad.sin() * n_dual
        };

        self.pos = r * self.original_pos;
    }

    pub fn to_triangles(self) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = vec![];
        let mut vertices: Vec<Vec<Vertex>> = vec![vec![]; 7];

        let equator: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        let north30: Vec3 = Self::rot_z(f32::consts::PI /  6.0) * equator;
        let north60: Vec3 = Self::rot_z(f32::consts::PI /  3.0) * equator;
        let north90: Vec3 = Self::rot_z(f32::consts::PI /  2.0) * equator;
        let south30: Vec3 = Self::rot_z(f32::consts::PI / -6.0) * equator;
        let south60: Vec3 = Self::rot_z(f32::consts::PI / -3.0) * equator;
        let south90: Vec3 = Self::rot_z(f32::consts::PI / -2.0) * equator;

        // rotate around y, append theses new vectors in

        for i in 0..=8 {
            let angle: f32 = f32::consts::PI / 4.0 * i as f32;
            let rot: Mat3 = Self::rot_y(angle);

            let n90: Vec3 = rot * north90;
            let n60: Vec3 = rot * north60;
            let n30: Vec3 = rot * north30;
            let eq0: Vec3 = rot * equator;
            let s30: Vec3 = rot * south30;
            let s60: Vec3 = rot * south60;
            let s90: Vec3 = rot * south90;

            vertices[0].push(Vertex::new(n90.x, n90.y, n90.z, self.color));
            vertices[1].push(Vertex::new(n60.x, n60.y, n60.z, self.color));
            vertices[2].push(Vertex::new(n30.x, n30.y, n30.z, self.color));
            vertices[3].push(Vertex::new(eq0.x, eq0.y, eq0.z, self.color));
            vertices[4].push(Vertex::new(s30.x, s30.y, s30.z, self.color));
            vertices[5].push(Vertex::new(s60.x, s60.y, s60.z, self.color));
            vertices[6].push(Vertex::new(s90.x, s90.y, s90.z, self.color));
        }

        // Scale and move vertices to correct world position

        let transform: Mat4 = Mat4::from_cols(
            Vec4::new(self.radius, 0.0, 0.0, 0.0,), 
            Vec4::new(0.0, self.radius, 0.0, 0.0,), 
            Vec4::new(0.0, 0.0, self.radius, 0.0,), 
            self.pos.extend(1.0),
        );

        for i in 0..vertices.len() {
            for j in 0..vertices[i].len() {
                vertices[i][j].pos = transform * vertices[i][j].pos;
            }
        }

        // construct triangles from those vertices
        
        for j in 0..8 {
            for i in 1..vertices.len() - 1 {
                triangles.push(Triangle::new(vertices[i][j + 1], vertices[i][j], vertices[i + 1][j + 1]));
                triangles.push(Triangle::new(vertices[i][j], vertices[i][j + 1], vertices[i - 1][j]));
            }
        }

        triangles
    }
}

pub struct CosmicSimulator {
    pub planets: Vec<CosmicBody>,
    pub sun: CosmicBody,
    pub days_passed: Vec<u32>,
}

impl CosmicSimulator {
    // ORIGINAL DATA (DISTANCES AND RING COUNT ARE REAL)
    //
    // Planet,     Distance from Sun (Avg. AU),    Orbital Period (Earth Days),    Radius (km),    Surface Color (Approx. RGB),    Has Ring (Count)
    // Mercury,    0.39,                           88,                             "2,440",        "[165, 155, 154] Gray",         0
    // Venus,      0.72,                           224.7,                          "6,052",        "[227, 158, 28] Yellow-White",  0
    // Earth,      1.00,                           365.2,                          "6,371",        "[43, 101, 236] Blue/Green",    0
    // Mars,       1.52,                           687,                            "3,390",        "[193, 68, 14] Red-Orange",     0
    // Jupiter,    5.20,                           "4,331",                        "69,911",       "[216, 202, 157] Brown/Tan",    4
    // Saturn,     9.54,                           "10,747",                       "58,232",       "[191, 171, 119] Pale Gold",    7 (Main Groups)
    // Uranus,     19.22,                          "30,589",                       "25,362",       "[209, 231, 231] Pale Blue",    13
    // Neptune,    30.06,                          "59,800",                       "24,622",       "[63, 115, 255] Bright Blue",   5

    /* This is the design choice detail
     * I decide to keep the radius squared scale linearly to the original data's radius
     * And make further bodies viewable

    Body       | Radius     | Orbit Center    | Gap to Prev
    ------------------------------------------------------------
    Sun        | 48.44      | 0.0000          | N/A
    Mercury    | 9.29       | 78.2239         | 20.5
    Venus      | 12.04      | 120.0475        | 20.5
    Earth      | 12.22      | 164.8008        | 20.5
    Mars       | 10.20      | 207.7171        | 20.5
    Jupiter    | 24.22      | 262.6368        | 20.5
    Saturn     | 22.99      | 330.3424        | 20.5
    Uranus     | 18.13      | 391.9565        | 20.5
    Neptune    | 17.97      | 448.5588        | 20.5

    */

    pub fn new()->Self {
        let mut planets: Vec<CosmicBody> = vec![
            // Mercury
            CosmicBody::new(Vec3::X *  102.4429,    88, Vec3::Z, Color::new(165, 155, 154), 9.29),
            // Venus
            CosmicBody::new(Vec3::X *  144.2665,   225, Vec3::Z, Color::new(227, 158,  28), 12.04),
            // Earth
            CosmicBody::new(Vec3::X *  189.0198,   365, Vec3::Z, Color::new( 43, 101, 236), 12.22),
            // Mars
            CosmicBody::new(Vec3::X * 231.9361,   687, Vec3::Z, Color::new(193,  68,  14), 10.20),
            // Jupiter
            CosmicBody::new(Vec3::X * 286.8558,  4331, Vec3::Z, Color::new(216, 202, 157), 24.22),
            // Saturn
            CosmicBody::new(Vec3::X * 354.5613, 10747, Vec3::Z, Color::new(191, 171, 119), 22.99),
            // Uranus
            CosmicBody::new(Vec3::X * 416.1755, 30589, Vec3::Z, Color::new(209, 231, 231), 18.13),
            // Neptune
            CosmicBody::new(Vec3::X * 472.7778, 59800, Vec3::Z, Color::new( 63, 115, 255), 17.97),
        ];
        
        let mut days_passed: Vec<u32> = vec![0; 8];

        // randomize original position
        let mut rng = rand::rng();
        for i in 0..8 {
            let days_passed_curr: u32 = rng.random_range(0..planets[i].days_per_orbit);
            days_passed[i] = days_passed_curr;
            planets[i].orbit(days_passed_curr);
        }

        let sun: CosmicBody = CosmicBody::new(Vec3::ZERO, 0, Vec3::ZERO, Color::new(255, 215, 0), 72.66);

        Self { planets, sun, days_passed }
    }

    pub fn orbit(&mut self, day: u32) {
        for i in 0..self.planets.len() {
            self.days_passed[i] += day;
            self.planets[i].orbit(self.days_passed[i]);
        }
    }

    pub fn to_triangles(&self) -> Vec<Triangle> {
        let mut vec: Vec<Triangle> = vec![];
        
        for planet in &self.planets {
            let mut planet_tri = planet.to_triangles();
            vec.append(&mut planet_tri);
        }
        let mut sun_tri = self.sun.to_triangles();
        vec.append(&mut sun_tri);

        vec
    }

}
