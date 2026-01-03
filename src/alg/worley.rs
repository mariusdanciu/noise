use crate::alg::{rand, Noise};
use glam::{Vec2, Vec3};

pub struct Worley {
    scale: f32,
}

impl Worley {
    pub fn new(scale: f32) -> Worley {
        Worley { scale }
    }
}

impl Noise for Worley {
    fn noise(&mut self, uv: Vec2, seed: f32) -> Vec3 {
        let st = uv * self.scale;

        let current_cell = st.floor();

        let mut min_dist = 1.0f32;

        for ny in -1..=1 {
            for nx in -1..=1 {
                let offset_cell = Vec2::new(nx as f32, ny as f32);

                let point = ((rand(current_cell + offset_cell, seed)) + 1.) * 0.5;

                let diff = (current_cell + offset_cell + point) - st;

                let dist = diff.length();

                min_dist = min_dist.min(dist);
            }
        }

        Vec3::new(min_dist, min_dist, min_dist)
    }
}
