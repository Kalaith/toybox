use super::library::{darken, draw_robot_core, draw_wheel};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);
    for offset in [-0.18_f32, 0.0, 0.18] {
        draw_cube(
            center + vec3(offset, -0.16, 0.04) * scale,
            vec3(0.08, 0.26, 0.08) * scale,
            None,
            darken(color, 0.16),
        );
        draw_wheel(
            center + vec3(offset, -0.30, 0.04) * scale,
            0.050 * scale,
            0.020 * scale,
            Color::new(0.12, 0.13, 0.14, 1.0),
        );
    }
}
