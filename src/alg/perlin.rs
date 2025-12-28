use glam::{Vec2, Vec3};
use rand::prelude::*;

use crate::alg::{mix_f32, Noise};

fn quintic(p: Vec2) -> Vec2 {
    return p * p * p * (p * (p * 6.0 - 15.) + 10.);
}

pub struct Perlin {
    pub scale: u32,
    pub anchors: Vec<Vec2>,
}

impl Perlin {
    fn rnd_anchors(scale: u32) -> Vec<Vec2> {
        let num = (scale + 1) * (scale + 1);
        let mut anchors: Vec<Vec2> = Vec::with_capacity((num) as usize);
        let mut rng = rand::rng();
        for _ in 0..num {
            let rx = rng.random_range(-1.0..1.0);
            let ry = rng.random_range(-1.0..1.0);

            anchors.push(Vec2::new(rx, ry));
        }

        anchors
    }

    pub fn new(scale: u32) -> Perlin {
        Perlin {
            scale,
            anchors: Perlin::rnd_anchors(scale),
        }
    }
}

impl Noise for Perlin {
    fn noise(&self, uv: Vec2) -> Vec3 {
        let s_uv = uv * self.scale as f32;

        let grid_id = s_uv.floor();
        let mut grid_uv = s_uv.fract();

        let index_tl = (grid_id.x + grid_id.y * (self.scale + 1) as f32) as usize;
        let index_tr = index_tl + 1;
        let index_br = index_tl + self.scale as usize + 2;
        let index_bl = index_br - 1;

        let grad_tl = self.anchors.get(index_tl).unwrap();
        let grad_tr = self.anchors.get(index_tr).unwrap();
        let grad_br = self.anchors.get(index_br).unwrap();
        let grad_bl = self.anchors.get(index_bl).unwrap();

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
