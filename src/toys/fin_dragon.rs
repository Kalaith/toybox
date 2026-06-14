use super::library::{draw_cube_with_edges, draw_dragon_base};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);
    let fin = Color::new(0.96, 0.88, 0.58, 1.0);
    for index in 0..4 {
        draw_cube_with_edges(
            center + vec3(0.0, 0.29 - index as f32 * 0.04, -0.12 + index as f32 * 0.13) * scale,
            vec3(0.08, 0.14, 0.05) * scale,
            fin,
        );
    }
    draw_cube(
        center + vec3(0.0, 0.12, 0.24) * scale,
        vec3(0.11, 0.10, 0.045) * scale,
        None,
        fin,
    );
}
