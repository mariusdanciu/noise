use glam::{Vec2, Vec3, Vec3Swizzles};

pub mod fbm;
pub mod perlin;
pub mod value;
pub mod worley;

const V1: Vec3 = Vec3::new(123.4, 234.5, 317.2);
const V2: Vec3 = Vec3::new(234.5, 345.6, 611.7);
const V3: Vec3 = Vec3::new(421.2, 314.7, 129.6);

fn quintic(p: Vec2) -> Vec2 {
    return p * p * p * (p * (p * 6.0 - 15.) + 10.);
}

fn quintic3D(p: Vec3) -> Vec3 {
    return p * p * p * (p * (p * 6.0 - 15.) + 10.);
}

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

pub fn rand(p: Vec2, seed: f32) -> Vec2 {
    let p = p + 0.02;
    let x = p.dot(V1.xy());
    let y = p.dot(V2.xy());
    let mut noise = Vec2::new(x, y);

    noise = Vec2::new(f32::sin(noise.x), f32::sin(noise.y));
    noise = noise * 43758.5453;

    noise = Vec2::new(f32::sin(noise.x + seed), f32::sin(noise.y + seed));
    return noise;
}

pub fn rand3D(p: Vec3, seed: f32) -> Vec3 {
    let p = p + 0.02;
    let x = p.dot(V1);
    let y = p.dot(V2);
    let z = p.dot(V3);
    let mut noise = Vec3::new(x, y, z);

    noise = Vec3::new(f32::sin(noise.x), f32::sin(noise.y), f32::sin(noise.z));
    noise = noise * 43758.5453;

    noise = Vec3::new(f32::sin(noise.x + seed), f32::sin(noise.y + seed), f32::sin(noise.z + seed));
    return noise;
}

pub fn rand_f32(p: Vec2, seed: f32) -> f32 {
    let mut noise = p.dot(Vec2::new(12.9898, 78.233));

    noise = f32::sin(noise);
    noise = noise * 43758.5453;

    noise = f32::sin(noise + seed);
    return noise;
}

pub trait Noise {
    fn noise(&self, uv: Vec2, freq: f32, seed: f32) -> Vec3;
    fn rescale_01(&self) -> bool;
}

pub trait Noise3D {
    fn noise(&self, uv: Vec3, freq: f32, seed: f32) -> Vec3;
}
