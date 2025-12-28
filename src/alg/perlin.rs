use glam::{Vec2, Vec3};

pub fn noise(uv: Vec2, scale: u32, anchors: &Vec<Vec2>) -> Vec3 {
    let s_uv = uv * scale as f32;

    let grid_id = s_uv.floor();
    let grid_uv = s_uv.fract();

    let min_dist = Vec3::new(grid_uv.x, grid_uv.y, 0.);
    min_dist
}
