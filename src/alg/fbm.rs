use glam::{Vec2, Vec3};

use crate::alg::Noise;

pub fn fbm(v: Vec2, noises: &mut Vec<impl Noise>, seed: f32) -> Vec3 {
    let mut sum = Vec3::ZERO;
    let mut amplitude = 1.0;
    let len = noises.len() as f32;

    for n in noises {
        sum += n.noise(v, seed) * amplitude;
        amplitude *= 0.5;
    }
    sum
}
