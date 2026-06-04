use super::library::{darken, draw_robot_core};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);
    draw_cube(
        center + vec3(-0.28, 0.12, -0.09) * scale,
        vec3(0.08, 0.26, 0.08) * scale,
        None,
        darken(color, 0.12),
    );
    draw_cube(
        center + vec3(0.28, 0.12, -0.09) * scale,
        vec3(0.08, 0.26, 0.08) * scale,
        None,
        darken(color, 0.12),
    );
    let metal = Color::new(0.90, 0.92, 0.92, 1.0);
    draw_cube(
        center + vec3(-0.34, -0.04, -0.12) * scale,
        vec3(0.16, 0.05, 0.05) * scale,
        None,
        metal,
    );
    draw_cube(
        center + vec3(0.34, -0.04, -0.12) * scale,
        vec3(0.16, 0.05, 0.05) * scale,
        None,
        metal,
    );
}
