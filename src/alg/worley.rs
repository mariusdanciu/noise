use glam::{Vec2, Vec3};
use rand::prelude::*;

use crate::alg::Noise;

pub struct Worley {
    scale: u32,
    anchors: Vec<Vec2>,
}

impl Worley {
    fn rnd_anchors(scale: u32) -> Vec<Vec2> {
        let mut anchors: Vec<Vec2> = Vec::with_capacity((scale * scale) as usize);
        let mut rng = rand::rng();
        for _ in 0..scale * scale {
            let rx = rng.random::<f32>();
            let ry = rng.random::<f32>();

            anchors.push(Vec2::new(rx, ry));
        }

        anchors
    }

    pub fn new(scale: u32) -> Worley {
        Worley {
            scale,
            anchors: Worley::rnd_anchors(scale),
        }
    }
}

impl Noise for Worley {
    fn noise(&self, uv: Vec2) -> Vec3 {
        let st = uv * self.scale as f32;

        let current_cell = Vec2::new(st.x.floor(), st.y.floor());

        let mut min_dist = 1.0f32;

        for ny in -1..=1 {
            for nx in -1..=1 {
                let offset_cell = Vec2::new(nx as f32, ny as f32);

                let index = ((current_cell.x + offset_cell.x)
                    + (current_cell.y + offset_cell.y) * self.scale as f32)
                    as usize;

                let point: Vec2;
                match self.anchors.get(index) {
                    None => continue,
                    Some(p) => point = *p,
                }

                let diff = (current_cell + offset_cell + point) - st;

                let dist = diff.length();

                min_dist = min_dist.min(dist);
            }
        }
        Vec3::new(min_dist, min_dist, min_dist)
    }
}
