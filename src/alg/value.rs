use glam::{Vec2, Vec3};

use crate::alg::{mix_f32, quintic, rand_f32, Noise};

pub struct Value {}

impl Value {
    pub fn new() -> Value {
        Value {}
    }
}

impl Noise for Value {
    fn noise(&self, uv: Vec2, freq: f32, seed: f32) -> f32 {
        let s_uv = uv * freq;

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

        let noise = mix_f32(t, b, grid_uv.y) * 0.5;

        noise
    }
    
    fn rescale_01(&self) ->bool {
        true
    }
}
