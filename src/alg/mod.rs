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

pub fn rand(p: Vec2) -> Vec2 {
    let p = p + 0.02;
    let x = p.dot(Vec2::new(123.4, 234.5));
    let y = p.dot(Vec2::new(234.5, 345.6));
    let mut noise = Vec2::new(x, y);

    noise = Vec2::new(f32::sin(noise.x), f32::sin(noise.y));
    noise = noise * 43758.5453;

    noise = Vec2::new(f32::sin(noise.x), f32::sin(noise.y));
    return noise * 1.01;
}

pub trait Noise {
    fn noise(&mut self, uv: Vec2) -> Vec3;
}
