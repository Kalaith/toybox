use super::library::draw_toy_sphere;
use super::library::{brighten, darken, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let beak = Color::new(0.96, 0.56, 0.18, 1.0);
    draw_toy_sphere(center, 0.25 * scale, None, color);
    draw_toy_sphere(
        center + vec3(0.0, -0.02, -0.11) * scale,
        0.14 * scale,
        None,
        brighten(color, 0.12),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.19, -0.25) * scale,
        0.15 * scale,
        None,
        brighten(color, 0.05),
    );
    draw_cube(
        center + vec3(0.0, 0.17, -0.41) * scale,
        vec3(0.20, 0.06, 0.12) * scale,
        None,
        beak,
    );
    draw_cube(
        center + vec3(-0.20, 0.03, -0.02) * scale,
        vec3(0.08, 0.16, 0.28) * scale,
        None,
        darken(color, 0.08),
    );
    draw_cube(
        center + vec3(0.20, 0.03, -0.02) * scale,
        vec3(0.08, 0.16, 0.28) * scale,
        None,
        darken(color, 0.08),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.04, 0.25) * scale,
        0.065 * scale,
        None,
        darken(color, 0.04),
    );
    draw_cube(
        center + vec3(-0.10, -0.21, -0.12) * scale,
        vec3(0.14, 0.04, 0.18) * scale,
        None,
        beak,
    );
    draw_cube(
        center + vec3(0.10, -0.21, -0.12) * scale,
        vec3(0.14, 0.04, 0.18) * scale,
        None,
        beak,
    );
    draw_face(center, 0.22, -0.37, 0.06, scale);
}
