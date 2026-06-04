use super::library::{darken, draw_robot_core};
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
    }
}
