use glam::{Vec2, Vec3};

use crate::alg::{mix_f32, quintic, rand, Noise};

pub struct Perlin {}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {}
    }
}

impl Noise for Perlin {
    fn noise(&self, uv: Vec2, freq: f32, seed: f32) -> Vec3 {
        let s_uv = uv * freq;

        let grid_id = s_uv.floor();
        let mut grid_uv = s_uv - grid_id;

        let tl = grid_id + Vec2::new(0.0, 0.0);
        let tr = grid_id + Vec2::new(1.0, 0.0);
        let br = grid_id + Vec2::new(1.0, 1.0);
        let bl = grid_id + Vec2::new(0.0, 1.0);

        let grad_tl = rand(tl, seed);
        let grad_tr = rand(tr, seed);
        let grad_br = rand(br, seed);
        let grad_bl = rand(bl, seed);

        let uv_to_tl = grid_uv - Vec2::new(0.0, 0.0);
        let uv_to_tr = grid_uv - Vec2::new(1.0, 0.0);
        let uv_to_br = grid_uv - Vec2::new(1.0, 1.0);
        let uv_to_bl = grid_uv - Vec2::new(0.0, 1.0);

        let dot_tl = grad_tl.dot(uv_to_tl);
        let dot_tr = grad_tr.dot(uv_to_tr);
        let dot_br = grad_br.dot(uv_to_br);
        let dot_bl = grad_bl.dot(uv_to_bl);

        grid_uv = quintic(grid_uv);

        let t = mix_f32(dot_tl, dot_tr, grid_uv.x);
        let b = mix_f32(dot_bl, dot_br, grid_uv.x);

        let noise = mix_f32(t, b, grid_uv.y);

        Vec3::new(noise, noise, noise)
    }

    fn rescale_01(&self) -> bool {
        true
    }
}
