use super::library::{draw_cube_with_edges, draw_dragon_base};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);
    let spike = Color::new(0.96, 0.88, 0.58, 1.0);
    for index in 0..5 {
        draw_cube_with_edges(
            center + vec3(0.0, 0.31, -0.24 + index as f32 * 0.11) * scale,
            vec3(0.07, 0.10, 0.07) * scale,
            spike,
        );
    }
}
