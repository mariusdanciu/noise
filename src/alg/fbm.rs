use glam::{Vec2, Vec3};

use crate::alg::Noise;

pub fn fbm(
    v: Vec2,
    octaves: u16,
    noise_alg: &impl Noise,
    persistence: f32,
    lacunarity: f32,
    seed: f32,
) -> Vec3 {
    let mut sum = Vec3::ZERO;
    let mut amplitude = 1.0;
    let mut freq = 1.0;

    for n in 0..octaves {
        sum += noise_alg.noise(v, freq, seed) * amplitude;
        amplitude *= persistence;
        freq *= lacunarity;
    }
    sum
}
