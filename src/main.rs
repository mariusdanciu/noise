mod alg;

use glam::Vec2;

use image::{ImageBuffer, Rgb};

use crate::alg::fbm::fbm;
use crate::alg::mix_vec3;
use crate::alg::perlin::Perlin;
use crate::alg::worley::Worley;
use crate::alg::Noise;

fn generate(
    res: u32,
    noises: Vec<impl Noise>,
    mut f: impl FnMut(u32, u32, f32, f32, f32),
) -> Vec<u8> {
    for iy in 0..res {
        for ix in 0..res {
            let p = Vec2::new(ix as f32, iy as f32);
            let uv = p / res as f32;

            let col = fbm(uv, &noises);

            let rgb = mix_vec3(0., 266., col);

            f(ix, iy, rgb.x, rgb.y, rgb.z);
        }
    }

    vec![]
}

fn main() {
    let res = 500;
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::new(res, res);

    // Different random points for each octave.
    let noises = vec![
        Perlin::new(4),
        Perlin::new(8),
        Perlin::new(16),
        Perlin::new(32),
    ];

    generate(res, noises, |ix, iy, r, g, b| {
        let pixel = imgbuf.get_pixel_mut(ix, iy);
        *pixel = image::Rgb([r as u8, g as u8, b as u8]);
    });

    imgbuf.save("out.png").unwrap();
}
