use std::ops::{Add, Mul, Sub};

use glam::{Vec2, Vec3};

pub mod fbm;
pub mod perlin;
pub mod worley;

pub fn mix_vec3(start: f32, stop: f32, a: Vec3) -> Vec3 {
    Vec3::new(
        start * (1. - a.x) + stop * a.x,
        start * (1. - a.y) + stop * a.y,
        start * (1. - a.z) + stop * a.z,
    )
}

pub fn mix_f32(start: f32, stop: f32, a: f32) -> f32 {
    start * (1. - a) + stop * a
}

pub trait Noise {
    fn noise(&self, uv: Vec2) -> Vec3;
}
