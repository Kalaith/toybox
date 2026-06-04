use super::library::{brighten, darken, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_sphere(center, 0.25 * scale, None, color);
    draw_sphere(
        center + vec3(0.0, 0.22, -0.15) * scale,
        0.16 * scale,
        None,
        brighten(color, 0.05),
    );
    draw_cube(
        center + vec3(-0.18, 0.20, -0.14) * scale,
        vec3(0.09, 0.24, 0.08) * scale,
        None,
        darken(color, 0.10),
    );
    draw_cube(
        center + vec3(0.18, 0.20, -0.14) * scale,
        vec3(0.09, 0.24, 0.08) * scale,
        None,
        darken(color, 0.10),
    );
    draw_sphere(
        center + vec3(0.0, 0.16, -0.29) * scale,
        0.08 * scale,
        None,
        Color::new(0.92, 0.78, 0.60, 1.0),
    );
    draw_cube(
        center + vec3(-0.21, 0.00, 0.16) * scale,
        vec3(0.15, 0.07, 0.07) * scale,
        None,
        color,
    );
    draw_face(center, 0.23, -0.32, 0.07, scale);
}
