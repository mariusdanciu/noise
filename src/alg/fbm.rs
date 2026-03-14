use glam::{Vec2, Vec3};

use crate::alg::{Noise, Noise3D};

pub fn fbm(
    v: Vec2,
    octaves: u16,
    noise_alg: &impl Noise,
    persistence: f32,
    lacunarity: f32,
    start_freq: f32,
    seed: f32,
) -> f32 {
    let mut sum = 0.0f32;
    let mut amplitude = 1.0;
    let mut freq = start_freq;

    for _ in 0..octaves {
        sum += noise_alg.noise(v, freq, seed) * amplitude;
        amplitude *= persistence;
        freq *= lacunarity;
    }
    sum
}

pub fn fbm_3d(
    v: Vec3,
    octaves: u16,
    noise_alg: &impl Noise3D,
    persistence: f32,
    lacunarity: f32,
    start_freq: f32,
    seed: f32,
) -> f32 {
    let mut sum = 0.0f32;
    let mut amplitude = 1.0;
    let mut freq = start_freq;

    for _ in 0..octaves {
        sum += noise_alg.noise(v, freq, seed) * amplitude;
        amplitude *= persistence;
        freq *= lacunarity;
    }
    sum
}
