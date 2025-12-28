use glam::{Vec2, Vec3};

use crate::alg::worley::noise;

pub fn fbm(v: Vec2, scale: u32, anchors: &Vec<Vec<Vec2>>) -> Vec3 {
    let mut sum = Vec3::ZERO;
    let mut amplitude = 1.0;
    let octaves = 4;

    let mut s = scale;

    for o in 0..octaves {
        sum += noise(v, s, anchors.get(o).unwrap()) * amplitude;
        amplitude *= 0.5;
        s *= 2;
    }
    sum
}
