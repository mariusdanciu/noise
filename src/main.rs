mod alg;

use glam::{Vec2, Vec3};

use image::{ImageBuffer, Rgb};

use crate::alg::fbm::fbm;
use crate::alg::{mix_vec3, worley};
use crate::alg::perlin::Perlin;
use crate::alg::worley::Worley;
use crate::alg::Noise;

fn generate(
    res: u32,
    offset: Vec2,
    noises: &mut Vec<impl Noise>,
    mut f: impl FnMut(u32, u32, Vec3),
) -> Vec<u8> {
    for iy in 0..res {
        for ix in 0..res {
            let p = Vec2::new(ix as f32, iy as f32);
            let uv = p / res as f32 + offset;

            let col = fbm(uv, noises);

            f(ix, iy, col);
        }
    }

    vec![]
}

fn main() {
    let res = 500;
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::new(res, res);

    // Different random points for each octave.
    let noises = &mut vec![
        Worley::new(4),
        Worley::new(8),
        Worley::new(16),
        Worley::new(32),
    ];

    let reverse_col = true;

    generate(res, Vec2::new(0., 0.), noises, |ix, iy, col| {
        let pixel = imgbuf.get_pixel_mut(ix, iy);

        let rgb = if !reverse_col {
            mix_vec3(0., 255., col)
        } else {
            mix_vec3(255., 0., col)
        };

        *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
    });

    imgbuf.save("out.png").unwrap();
}
