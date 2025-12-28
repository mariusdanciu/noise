use glam::{Vec2, Vec3};

pub fn noise(uv: Vec2, scale: u32, anchors: &Vec<Vec2>) -> Vec3 {
    let st = uv * scale as f32;

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
    Vec3::new(min_dist, min_dist, min_dist)
}
