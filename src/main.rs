use glam::Vec2;

use image::{ImageBuffer, Rgb};
use rand::prelude::*;

fn mix(start: f32, stop: f32, a: f32) -> f32 {
    start * (1. - a) + stop * a
}

fn anchors(scale: u32) -> Vec<Vec2> {
    let mut anchors: Vec<Vec2> = Vec::with_capacity((scale * scale) as usize);
    let mut rng = rand::rng();
    for _ in 0..scale * scale {
        let rx = rng.random::<f32>();
        let ry = rng.random::<f32>();

        anchors.push(Vec2::new(rx, ry));
    }

    anchors
}

fn worley_noise(p: Vec2, scale: u32, res: u32, anchors: &Vec<Vec2>) -> f32 {
    let mut st = p / res as f32;

    st.x *= scale as f32;
    st.y *= scale as f32;

    let current_cell = Vec2::new(st.x.floor(), st.y.floor());

    let mut min_dist = 1.0f32;

    for ny in -1..=1 {
        for nx in -1..=1 {
            let offset_cell = Vec2::new(nx as f32, ny as f32);

            let index = ((current_cell.x + offset_cell.x)
                + (current_cell.y + offset_cell.y) * scale as f32) as usize;

            let point: Vec2;
            match anchors.get(index) {
                None => continue,
                Some(p) => point = *p,
            }

            let diff = (current_cell + offset_cell + point) - st;

            let dist = diff.length();

            min_dist = min_dist.min(dist);
        }
    }
    min_dist
}

fn main() {
    let res = 500;
    let scale = 5;
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::new(res, res);

    let anchors = &anchors(scale);

    for iy in 0..res {
        for ix in 0..res {
            let min_dist = worley_noise(Vec2::new(ix as f32, iy as f32), scale, res, anchors);
            let intensity = mix(255., 0., min_dist) as u8;

            let pixel = imgbuf.get_pixel_mut(ix, iy);
            *pixel = image::Rgb([intensity, intensity, intensity]);
        }
    }

    imgbuf.save("out.png").unwrap();
}
