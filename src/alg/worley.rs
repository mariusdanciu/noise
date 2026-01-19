use crate::alg::{rand, Noise};
use glam::{Vec2, Vec3};

pub struct Worley {}

impl Worley {
    pub fn new() -> Worley {
        Worley {}
    }
}

impl Noise for Worley {
    fn noise(&self, uv: Vec2, freq: f32, seed: f32) -> f32 {
        let st = uv * freq;

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

        min_dist
    }
    fn rescale_01(&self, noise: f32) -> f32 {
        noise
    }
}
