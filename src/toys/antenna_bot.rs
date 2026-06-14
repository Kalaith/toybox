use super::library::{darken, draw_cube_with_edges, draw_robot_arms, draw_robot_core};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);
    draw_cube_with_edges(
        center + vec3(0.0, 0.58, -0.01) * scale,
        vec3(0.04, 0.22, 0.04) * scale,
        Color::new(0.10, 0.12, 0.14, 1.0),
    );
    draw_sphere(
        center + vec3(0.0, 0.72, -0.01) * scale,
        0.06 * scale,
        None,
        Color::new(0.95, 0.76, 0.30, 1.0),
    );
    draw_robot_arms(center, color, scale);
    draw_cube_with_edges(
        center + vec3(0.0, -0.14, 0.0) * scale,
        vec3(0.32, 0.08, 0.28) * scale,
        darken(color, 0.16),
    );
}
