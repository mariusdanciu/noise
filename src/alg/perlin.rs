use glam::{Vec2, Vec3};

use crate::alg::{mix_f32, quintic, quintic3D, rand, rand3D, Noise, Noise3D};

pub struct Perlin {}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {}
    }
}

impl Noise for Perlin {
    fn noise(&self, uv: Vec2, freq: f32, seed: f32) -> f32 {
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

        noise
    }

    fn rescale_01(&self) -> bool {
        true
    }
}

impl Noise3D for Perlin {
    fn noise(&self, uv: Vec3, freq: f32, seed: f32) -> f32 {
        let s_uv = uv * freq;

        let grid_id = s_uv.floor();
        let mut grid_uv = s_uv - grid_id;

        let tl_front = grid_id + Vec3::new(0.0, 0.0, 0.0);
        let tr_front = grid_id + Vec3::new(1.0, 0.0, 0.0);
        let br_front = grid_id + Vec3::new(1.0, 1.0, 0.0);
        let bl_front = grid_id + Vec3::new(0.0, 1.0, 0.0);

        let tl_back = grid_id + Vec3::new(0.0, 0.0, 1.0);
        let tr_back = grid_id + Vec3::new(1.0, 0.0, 1.0);
        let br_back = grid_id + Vec3::new(1.0, 1.0, 1.0);
        let bl_back = grid_id + Vec3::new(0.0, 1.0, 1.0);

        let grad_tl_front = rand3D(tl_front, seed);
        let grad_tr_front = rand3D(tr_front, seed);
        let grad_br_front = rand3D(br_front, seed);
        let grad_bl_front = rand3D(bl_front, seed);

        let grad_tl_back = rand3D(tl_back, seed);
        let grad_tr_back = rand3D(tr_back, seed);
        let grad_br_back = rand3D(br_back, seed);
        let grad_bl_back = rand3D(bl_back, seed);

        let uv_to_tl_front = grid_uv - Vec3::new(0.0, 0.0, 0.0);
        let uv_to_tr_front = grid_uv - Vec3::new(1.0, 0.0, 0.0);
        let uv_to_br_front = grid_uv - Vec3::new(1.0, 1.0, 0.0);
        let uv_to_bl_front = grid_uv - Vec3::new(0.0, 1.0, 0.0);

        let uv_to_tl_back = grid_uv - Vec3::new(0.0, 0.0, 1.0);
        let uv_to_tr_back = grid_uv - Vec3::new(1.0, 0.0, 1.0);
        let uv_to_br_back = grid_uv - Vec3::new(1.0, 1.0, 1.0);
        let uv_to_bl_back = grid_uv - Vec3::new(0.0, 1.0, 1.0);

        let dot_tl_front = grad_tl_front.dot(uv_to_tl_front);
        let dot_tr_front = grad_tr_front.dot(uv_to_tr_front);
        let dot_br_front = grad_br_front.dot(uv_to_br_front);
        let dot_bl_front = grad_bl_front.dot(uv_to_bl_front);

        let dot_tl_back = grad_tl_back.dot(uv_to_tl_back);
        let dot_tr_back = grad_tr_back.dot(uv_to_tr_back);
        let dot_br_back = grad_br_back.dot(uv_to_br_back);
        let dot_bl_back = grad_bl_back.dot(uv_to_bl_back);

        grid_uv = quintic3D(grid_uv);

        let noise = mix_f32(
            mix_f32(
                mix_f32(dot_tl_front, dot_tr_front, grid_uv.x),
                mix_f32(dot_bl_front, dot_br_front, grid_uv.x),
                grid_uv.y,
            ),
            mix_f32(
                mix_f32(dot_tl_back, dot_tr_back, grid_uv.x),
                mix_f32(dot_bl_back, dot_br_back, grid_uv.x),
                grid_uv.y,
            ),
            grid_uv.z,
        );

        noise
    }
}
