use glam::{Vec2, Vec3};
use crate::alg::{rand, Noise};

pub struct Worley {
    scale: u32,
}

impl Worley {
    pub fn new(scale: u32) -> Worley {
        Worley { scale }
    }
}

impl Noise for Worley {
    fn noise(&mut self, uv: Vec2) -> Vec3 {
        let st = uv * self.scale as f32;

        let current_cell = st.floor();

        let mut min_dist = 1.0f32;

        for ny in -1..=1 {
            for nx in -1..=1 {
                let offset_cell = Vec2::new(nx as f32, ny as f32);

                let point = (rand(current_cell + offset_cell)).abs();

                let diff = (current_cell + offset_cell + point) - st;

                let dist = diff.length();

                min_dist = min_dist.min(dist);
            }
        }

        Vec3::new(min_dist, min_dist, min_dist)
    }
}
