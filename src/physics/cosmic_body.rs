use glam::{Vec3};

use crate::graphics::triangle::Color;

#[derive(Clone, Copy)]
pub struct CosmicBody {
    // Unit:
    //  length:     AU,
    //  mass:       Solar mass,
    //  time:       day,
    // All others derive from here

    pub pos: Vec3,
    pub vel: Vec3,
    pub mass: f32,
    pub color: Color,
    pub radius: f32,
}

impl CosmicBody {
    pub fn new(pos: Vec3, vel: Vec3, mass: f32, color: Color, radius: f32) -> Self {
        Self { pos, vel, mass, color, radius, }
    }
}

pub struct CosmicSimulator {
    pub planets: Vec<CosmicBody>,
    pub sun: CosmicBody,
}

impl CosmicSimulator {
    pub fn new()->Self {
        let planets: Vec<CosmicBody> = vec![
            // Earth
            CosmicBody::new(
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0172, 3e-6), 
                3.23e-7,
                Color::new(0, 102, 204), 
                0.1
            ),
        ];
        let sun: CosmicBody = CosmicBody::new(Vec3::ZERO, Vec3::ZERO, 1.0, Color::new(255, 215, 0), 0.25);

        Self { planets, sun }
    }
}
