use glam::{Vec2, Vec3};

use crate::alg::{mix_f32, quintic, rand_f32, Noise};

pub struct Value {
    pub scale: f32,
}

impl Value {
    pub fn new(scale: f32) -> Value {
        Value { scale }
    }
}

impl Noise for Value {
    fn noise(&mut self, uv: Vec2, seed: f32) -> Vec3 {
        let s_uv = uv * self.scale;

        let grid_id = s_uv.floor();
        let mut grid_uv = s_uv - grid_id;

        let tl = grid_id + Vec2::new(0.0, 0.0);
        let tr = grid_id + Vec2::new(1.0, 0.0);
        let br = grid_id + Vec2::new(1.0, 1.0);
        let bl = grid_id + Vec2::new(0.0, 1.0);

        let grad_tl = rand_f32(tl, seed);
        let grad_tr = rand_f32(tr, seed);
        let grad_br = rand_f32(br, seed);
        let grad_bl = rand_f32(bl, seed);

        grid_uv = quintic(grid_uv);

        let t = mix_f32(grad_tl, grad_tr, grid_uv.x);
        let b = mix_f32(grad_bl, grad_br, grid_uv.x);

        let noise = mix_f32(t, b, grid_uv.y) / 2.0;

        Vec3::new(noise, noise, noise)
    }
}
