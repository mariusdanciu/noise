
use glam::{Vec2, Vec3};

use image::{ImageBuffer, Rgb, Rgba};

use noise::alg::fbm::fbm;
use noise::alg::perlin::Perlin;
use noise::alg::value::Value;
use noise::alg::worley::Worley;
use noise::alg::Noise;
use noise::alg::{mix_vec3, worley};

fn generate(
    res: u32,
    seed: f32,
    offset: Vec2,
    noise: &impl Noise,
    mut f: impl FnMut(u32, u32, Vec3),
) -> Vec<u8> {
    for iy in 0..res {
        for ix in 0..res {
            let p = Vec2::new(ix as f32, iy as f32);
            let uv = p / res as f32 + offset;

            let mut col = fbm(uv, 6, noise, 0.5, 2.0, seed);
            if noise.rescale_01() {
                col = (col + 1.0) * 0.5;
            }
            f(ix, iy, col);
        }
    }

    vec![]
}

fn main() {
    let res = 500;
    let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<_>> = image::ImageBuffer::new(res, res);

    let a = 0.0f32; // 1 - inverse col, 0 - keep the exact col.

    let noise_alg = Perlin::new();
    generate(res, 25., Vec2::new(0., 0.0), &noise_alg, |ix, iy, col| {
        let pixel = imgbuf.get_pixel_mut(ix, iy);

        let rgb = mix_vec3(0., 255., (1. - col) * a + (1. - a) * col);

        *pixel = image::Rgba([rgb.x as u8, rgb.y as u8, rgb.z as u8, 255u8]);
    });

    imgbuf.save("out.png").unwrap();
}
