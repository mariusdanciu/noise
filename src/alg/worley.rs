use crate::alg::{rand, rand_3d, Noise, Noise3D};
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
        let local = st - current_cell;

        let mut min_dist = 1.0f32;

        for ny in -1..=1 {
            for nx in -1..=1 {
                let neighbor = Vec2::new(nx as f32, ny as f32);

                let point = ((rand(current_cell + neighbor, seed)) + 1.) * 0.5;

                let diff = neighbor + point - local;

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

impl Noise3D for Worley {
    fn noise(&self, uv: Vec3, freq: f32, seed: f32) -> f32 {
        let st = uv * freq;

        let current_cell = st.floor();
        let local = st - current_cell;

        let mut min_dist = 1.0f32;

        for nz in -1..=1 {
            for ny in -1..=1 {
                for nx in -1..=1 {
                    let neighbor = Vec3::new(nx as f32, ny as f32, nz as f32);

                    let point = ((rand_3d(current_cell + neighbor, seed)) + 1.) * 0.5;

                    //let diff = (current_cell + neighbor + point) - st;
                    let diff = neighbor + point - local;
                    // neighbor + point - st + current_cell

                    let dist = diff.length();

                    min_dist = min_dist.min(dist);
                }
            }
        }

        min_dist
    }

    fn rescale_01(&self, noise: f32) -> f32 {
        noise
    }
}
