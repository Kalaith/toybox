use super::library::{brighten, darken, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_sphere(center, 0.24 * scale, None, color);
    draw_sphere(
        center + vec3(0.0, 0.22, -0.16) * scale,
        0.16 * scale,
        None,
        brighten(color, 0.04),
    );
    draw_cube(
        center + vec3(-0.12, 0.39, -0.16) * scale,
        vec3(0.10, 0.12, 0.08) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.12, 0.39, -0.16) * scale,
        vec3(0.10, 0.12, 0.08) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.24, 0.10, 0.18) * scale,
        vec3(0.08, 0.28, 0.08) * scale,
        None,
        darken(color, 0.08),
    );
    draw_cube(
        center + vec3(0.0, 0.18, -0.31) * scale,
        vec3(0.05, 0.04, 0.05) * scale,
        None,
        Color::new(0.06, 0.04, 0.04, 1.0),
    );
    draw_face(center, 0.25, -0.31, 0.07, scale);
}
