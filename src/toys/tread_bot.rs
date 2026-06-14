use super::library::{draw_cube_with_edges, draw_robot_core};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);
    draw_cube_with_edges(
        center + vec3(0.0, -0.15, 0.0) * scale,
        vec3(0.46, 0.12, 0.34) * scale,
        Color::new(0.08, 0.09, 0.10, 1.0),
    );
    for offset in [-0.17_f32, 0.0, 0.17] {
        draw_cube(
            center + vec3(offset, -0.09, -0.19) * scale,
            vec3(0.08, 0.05, 0.06) * scale,
            None,
            Color::new(0.78, 0.82, 0.84, 1.0),
        );
    }
    for offset in [-0.17_f32, 0.0, 0.17] {
        draw_cube(
            center + vec3(offset, -0.09, 0.19) * scale,
            vec3(0.08, 0.05, 0.06) * scale,
            None,
            Color::new(0.78, 0.82, 0.84, 1.0),
        );
    }
}
