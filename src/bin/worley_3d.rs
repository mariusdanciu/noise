use std::fs::File;
use std::io::{self, Write};

use glam::{Vec2, Vec3, Vec3Swizzles};

use image::{ImageBuffer, Rgb, Rgba};

use noise::alg::fbm::fbm_3d;
use noise::alg::perlin::Perlin;
use noise::alg::value::Value;
use noise::alg::worley::Worley;
use noise::alg::{mix_f32, mix_vec3, worley};
use noise::alg::{Noise, Noise3D};

fn generate(res: u32, seed: f32, offset: Vec3, noise: &impl Noise3D) -> Vec<u8> {
    let mut volume = Vec::with_capacity((res * res * res) as usize);
    for iz in 0..res {
        for iy in 0..res {
            for ix in 0..res {
                let p = Vec3::new(ix as f32, iy as f32, iz as f32);
                let uv = p / res as f32 + offset;

                let col = fbm_3d(uv, 12, noise, 0.5, 2.0, 3.0, seed);

                let col = noise.rescale_01(col);
                let inv = 1.0 - col;
                let noise = (inv * 255.) as u8;
                volume.push(noise);
            }
        }
    }
    volume
}

fn slices_to_png(volume: &Vec<u8>, res: u32) {
    let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<_>> = image::ImageBuffer::new(res, res);

    for z in [0, 3, 6, 9] {
        for y in 0..res {
            for x in 0..res {
                let pixel = imgbuf.get_pixel_mut(x, y);
                let noise = volume[((z * res * res) + (y * res) + x) as usize];
                *pixel = image::Rgba([noise, noise, noise, 255u8]);
            }
        }
        imgbuf.save(format!("out-{}.png", z)).unwrap();
    }
}
fn main() -> io::Result<()> {
    let res = 128;

    let noise_alg = Worley::new();

    let volume = generate(res, 191.1, Vec3::new(0., 0., 0.), &noise_alg);

    slices_to_png(&volume, res);

    let mut file = File::create("cloud_noise.raw")?;
    file.write_all(&volume)
}
