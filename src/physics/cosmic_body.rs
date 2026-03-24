use std::f32;
use std::f32::consts::PI;

use rand::{RngExt};
use glam::{Mat3, Mat4, Vec3, Vec4, Vec4Swizzles};

use crate::graphics::{projection::Camera, triangle::{Color, Material, Triangle, Vertex}};

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
    const MAT: Material = Material { ks: 0.0, ka: 2.0, p: 3.0 };

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

    pub fn to_triangles(&mut self, latitudes: usize, longtitudes: usize) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = vec![];
        let mut vertices: Vec<Vec<Vertex>> = vec![vec![]; latitudes + 1];
        let equator: Vec3 = Vec3::new(1.0, 0.0, 0.0);

        // create vertices on the 0th longitude (prime meridian)
        let mut prime_meridian_vertices: Vec<Vec3> = vec![];
        for i in 0..=latitudes {
            let current_rot: f32 = (PI / 2.0) - PI * (latitudes as f32) - i as f32;
            let v: Vec3 = Self::rot_y(current_rot) * equator;
            prime_meridian_vertices.push(v);
        }

        // For each longtitude, copy the latitudes in after rotation
        for j in 0..=longtitudes {
            let rot: Mat3 = Self::rot_z(PI * 2.0 / longtitudes as f32 * j as f32);
            for i in 0..=latitudes {
                let rotated: Vec3 = rot * prime_meridian_vertices[i];
                vertices[i].push(Vertex::new(rotated.x, rotated.y, rotated.z, self.color));
            }
        }

        // Scale/move vertices to correct world pos
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

        // construct triangles from thos vertices
        for j in 0..longtitudes {
            for i in 1..(vertices.len() - 1) {
                triangles.push(Triangle::new(vertices[i][j + 1], vertices[i][j], vertices[i + 1][j + 1], Self::MAT));
                triangles.push(Triangle::new(vertices[i][j], vertices[i][j + 1], vertices[i - 1][j], Self::MAT));
            }
        }

        triangles
    }
}

pub struct CosmicSimulator {
    pub planets: Vec<CosmicBody>,
    pub sun: CosmicBody,
    pub days_passed: Vec<u32>,
    pub orbit_triangles: Vec<Triangle>,
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
            CosmicBody::new(Vec3::X *  102.4429,    88, Vec3::Z, Color::new(165, 155, 154), 9.29 + 5.0),
            // Venus
            CosmicBody::new(Vec3::X *  144.2665,   225, Vec3::Z, Color::new(227, 158,  28), 12.04 + 5.0),
            // Earth
            CosmicBody::new(Vec3::X *  189.0198,   365, Vec3::Z, Color::new( 43, 101, 236), 12.22 + 5.0),
            // Mars
            CosmicBody::new(Vec3::X * 231.9361,   687, Vec3::Z, Color::new(193,  68,  14), 10.20 + 5.0),
            // Jupiter
            CosmicBody::new(Vec3::X * 286.8558,  4331, Vec3::Z, Color::new(216, 202, 157), 24.22 + 5.0),
            // Saturn
            CosmicBody::new(Vec3::X * 354.5613, 10747, Vec3::Z, Color::new(191, 171, 119), 22.99 + 5.0),
            // Uranus
            CosmicBody::new(Vec3::X * 416.1755, 30589, Vec3::Z, Color::new(209, 231, 231), 18.13 + 5.0),
            // Neptune
            CosmicBody::new(Vec3::X * 472.7778, 59800, Vec3::Z, Color::new( 63, 115, 255), 17.97 + 5.0),
        ];
        
        let mut days_passed: Vec<u32> = vec![0; 8];

        // randomize original position
        let mut rng = rand::rng();
        for i in 0..8 {
            let days_passed_curr: u32 = rng.random_range(0..planets[i].days_per_orbit);
            days_passed[i] = days_passed_curr;
            planets[i].orbit(days_passed_curr);
        }

        let sun: CosmicBody = CosmicBody::new(Vec3::ZERO, 0, Vec3::ZERO, Color::new(255, 215, 0), 72.66 + 5.0);

        let mut orbit_triangles: Vec<Triangle> = vec![];
        let orbit_line_counts: f32 = 1000.0;
        let mut orbit_color: Color = Color::WHITE;
        let orbit_line_width: f32 = 3.0;
        for planet in &mut planets.iter() {
            // construct a ring with radius 5, then rotate it around the x axis.
            let p1_0: Vec3 = planet.original_pos + Vec3::X * orbit_line_width / 2.0;
            let p2_0: Vec3 = planet.original_pos - Vec3::X * orbit_line_width / 2.0;

            orbit_color = planet.color;

            for i in 0..=(orbit_line_counts as u32) {
                let theta1: f32 = 2.0 * PI / orbit_line_counts * i as f32;
                let theta2: f32 = 2.0 * PI / orbit_line_counts * (i + 1) as f32;
                let p1_1: Vec3 = CosmicBody::rot_z(theta1) * p1_0;
                let p2_1: Vec3 = CosmicBody::rot_z(theta1) * p2_0;
                let p1_2: Vec3 = CosmicBody::rot_z(theta2) * p1_0;
                let p2_2: Vec3 = CosmicBody::rot_z(theta2) * p2_0;
                orbit_triangles.push(Triangle::new(
                        Vertex::from_vec3(p1_1, orbit_color),
                        Vertex::from_vec3(p1_2, orbit_color),
                        Vertex::from_vec3(p2_2, orbit_color),
                        CosmicBody::MAT,
                ));
                orbit_triangles.push(Triangle::new(
                        Vertex::from_vec3(p1_1, orbit_color),
                        Vertex::from_vec3(p2_2, orbit_color),
                        Vertex::from_vec3(p1_2, orbit_color),
                        CosmicBody::MAT
                ));
                orbit_triangles.push(Triangle::new(
                        Vertex::from_vec3(p1_1, orbit_color),
                        Vertex::from_vec3(p2_2, orbit_color),
                        Vertex::from_vec3(p2_1, orbit_color),
                        CosmicBody::MAT
                ));
                orbit_triangles.push(Triangle::new(
                        Vertex::from_vec3(p1_1, orbit_color),
                        Vertex::from_vec3(p2_1, orbit_color),
                        Vertex::from_vec3(p2_2, orbit_color),
                        CosmicBody::MAT
                ));
            }
        }

        Self { planets, sun, days_passed, orbit_triangles }
    }

    pub fn orbit(&mut self, day: u32) {
        for i in 0..self.planets.len() {
            self.days_passed[i] += day;
            self.planets[i].orbit(self.days_passed[i]);
        }
    }

    pub fn calculate_triangles(&mut self, camera: Camera) -> Vec<Triangle> {
        let mut vec: Vec<Triangle> = vec![];

        let l_ambient: Vec3 = Vec3::new(20.0, 20.0, 40.0) * 2.0;
        for planet in &mut self.planets {
            let mut planet_tri = planet.to_triangles(8, 16);

            
            for triangle in &mut planet_tri {
                let pos: Vec3 = (
                        triangle.a.pos.xyz()
                        + triangle.b.pos.xyz()
                        + triangle.c.pos.xyz()
                ) / 3.0;
                let n: Vec3 = -triangle.normal.xyz();
                let l: Vec3 = (self.sun.pos - pos).normalize();
                let v: Vec3 = (pos - camera.e.xyz()).normalize();
                let h: Vec3 = (n + v).normalize();
                let r2: f32 = (self.sun.pos - pos).dot(self.sun.pos - pos);
                let p: f32  = 3.0;

                let k_diffuse: Vec3 = (Color::to_vec3(triangle.a.rgb) + 
                                Color::to_vec3(triangle.b.rgb) +
                                Color::to_vec3(triangle.c.rgb)) / 3.0;
                let k_specular: f32 = 000.0;
                let intensity: f32 = 100000.0;


                let l_diffuse: Vec3 = k_diffuse * (intensity / r2) * n.dot(l).max(0.0);
                let l_specular: f32 = k_specular * (intensity / r2) * n.dot(h).max(0.0).powf(p);
                
                let final_rgb: Vec3 = l_ambient + l_diffuse + l_specular;

                // if n.dot(l) < 0.0 { panic!("Finally!") }
                let col: Color = Color::from_vec3(final_rgb);
                
                triangle.a.rgb = col;
                triangle.b.rgb = col;
                triangle.c.rgb = col;
            }
            

            vec.append(&mut planet_tri);
        }

        // sun-specific logic
        let mut sun_tri: Vec<Triangle> = self.sun.to_triangles(16, 16);
        for triangle in &mut sun_tri {
            let pos: Vec3 = (
                triangle.a.pos.xyz()
                + triangle.b.pos.xyz()
                + triangle.c.pos.xyz()
            ) / 3.0;
            // Inside your loop, if planet == sun:
            let n = triangle.normal.xyz();
            let v = (pos - camera.e.xyz()).normalize();

            // How much is the face looking at the camera?
            let view_alignment = v.dot(n).max(0.0);

            // Limb Darkening: Center is bright white-yellow, edges are deep orange
            let core_glow = 1.2; // Over-brighten the center
            let limb_factor = view_alignment.powf(0.5); // Soft falloff to the edges

            let final_rgb = triangle.a.rgb.to_vec3() * core_glow * limb_factor;

            triangle.a.rgb = Color::from_vec3(final_rgb);
            triangle.b.rgb = Color::from_vec3(final_rgb);
            triangle.c.rgb = Color::from_vec3(final_rgb);
        }
        vec.append(&mut sun_tri);

        vec.extend(self.orbit_triangles.iter().cloned());

        vec
    }
    
}

