use std::collections::HashSet;

use glam::{Vec2, Vec3};
use rand::prelude::*;

use crate::alg::{mix_f32, mix_vec3, Noise};

fn noise(p: Vec2) -> Vec2 {
    let p = p + 0.02;
    let x = p.dot(Vec2::new(123.4, 234.5));
    let y = p.dot(Vec2::new(234.5, 345.6));
    let mut noise = Vec2::new(x, y);

    noise = Vec2::new(f32::sin(noise.x), f32::sin(noise.y));
    noise = noise * 43758.5453;

    noise = Vec2::new(f32::sin(noise.x), f32::sin(noise.y));
    return noise * 1.01;
}

// vec2 noise2x2(vec2 p) {
//   float x = dot(p, vec2(123.4, 234.5));
//   float y = dot(p, vec2(345.6, 456.7));
//   vec2 noise = vec2(x, y);
//   noise = sin(noise);
//   noise = noise * 43758.5453;
//   noise = fract(noise);
//   return noise;
// }

fn quintic(p: Vec2) -> Vec2 {
    return p * p * p * (p * (p * 6.0 - 15.) + 10.);
}

pub struct Perlin {
    pub scale: u32,
}

impl Perlin {
    pub fn new(scale: u32) -> Perlin {
        Perlin { scale }
    }
}

impl Noise for Perlin {
    fn noise(&mut self, uv: Vec2) -> Vec3 {
        let s_uv = uv * self.scale as f32;

        let grid_id = s_uv.floor();
        let mut grid_uv = s_uv - grid_id;

        let s = format!("{},{}", grid_id.x, grid_id.y);

        let tl = grid_id + Vec2::new(0.0, 0.0);
        let tr = grid_id + Vec2::new(1.0, 0.0);
        let br = grid_id + Vec2::new(1.0, 1.0);
        let bl = grid_id + Vec2::new(0.0, 1.0);

        let grad_tl = noise(tl);
        let grad_tr = noise(tr);
        let grad_br = noise(br);
        let grad_bl = noise(bl);

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

        let noise = mix_f32(t, b, grid_uv.y) + 0.1;

        Vec3::new(noise, noise, noise)
    }
}
