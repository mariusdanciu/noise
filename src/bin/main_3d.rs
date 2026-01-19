use glam::{Vec2, Vec3, Vec3Swizzles};

use image::{ImageBuffer, Rgb, Rgba};

use noise::alg::fbm::fbm_3d;
use noise::alg::perlin::Perlin;
use noise::alg::value::Value;
use noise::alg::worley::Worley;
use noise::alg::{mix_f32, mix_vec3, worley};
use noise::alg::{Noise, Noise3D};

fn generate(
    res: u32,
    seed: f32,
    layer: u32,
    offset: Vec3,
    noise: &impl Noise3D,
    mut f: impl FnMut(u32, u32, u32, f32),
) -> Vec<u8> {
    let mut min = 1.0f32;
    let mut max = 0.0;
    for iy in 0..res {
        for ix in 0..res {
            let p = Vec3::new(ix as f32, iy as f32, layer as f32);
            let uv = p / res as f32 + offset;

            let col = fbm_3d(uv, 6, noise, 0.5, 2.0, seed);

            let col = noise.rescale_01(col);
            if col < min {
                min = col;
            }
            if col > max {
                max = col;
            }
            f(ix, iy, layer, col);
        }
    }

    println!("min {} max {}", min, max);
    vec![]
}

fn main() {
    let res = 500;
    let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<_>> = image::ImageBuffer::new(res, res);

    let noise_alg = Perlin::new();
    generate(
        res,
        115.,
        0,
        Vec3::new(0., 0.0, 0.0),
        &noise_alg,
        |ix, iy, iz, col| {
            let pixel = imgbuf.get_pixel_mut(ix, iy);

            let noise = (col * 255.) as u8;

            *pixel = image::Rgba([noise, noise, noise, 255u8]);
        },
    );

    imgbuf.save("out2.png").unwrap();
}
