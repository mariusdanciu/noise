mod alg;

use glam::Vec2;

use image::{ImageBuffer, Rgb};
use rand::prelude::*;

use crate::alg::fbm::fbm;
use crate::alg::mix_vec3;

fn rnd_anchors(scale: u32) -> Vec<Vec2> {
    let mut anchors: Vec<Vec2> = Vec::with_capacity((scale * scale) as usize);
    let mut rng = rand::rng();
    for _ in 0..scale * scale {
        let rx = rng.random::<f32>();
        let ry = rng.random::<f32>();

        anchors.push(Vec2::new(rx, ry));
    }

    anchors
}

fn main() {
    let res = 500;
    let scale = 4;
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::new(res, res);

    // Different random points for each octave.
    let anchors = vec![
        rnd_anchors(scale),
        rnd_anchors(scale * 2),
        rnd_anchors(scale * 4),
        rnd_anchors(scale * 6),
    ];

    for iy in 0..res {
        for ix in 0..res {
            let p = Vec2::new(ix as f32, iy as f32);
            let uv = p / res as f32;

            let col = fbm(uv, scale, &anchors);

            let rgb = mix_vec3(255., 0., col);

            let pixel = imgbuf.get_pixel_mut(ix, iy);
            *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
        }
    }

    imgbuf.save("out.png").unwrap();
}
