use super::library::{brighten, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_sphere(center, 0.26 * scale, None, color);
    draw_sphere(
        center + vec3(0.0, 0.22, -0.12) * scale,
        0.17 * scale,
        None,
        brighten(color, 0.06),
    );
    draw_sphere(
        center + vec3(-0.16, 0.36, -0.12) * scale,
        0.08 * scale,
        None,
        color,
    );
    draw_sphere(
        center + vec3(0.16, 0.36, -0.12) * scale,
        0.08 * scale,
        None,
        color,
    );
    draw_sphere(
        center + vec3(0.0, 0.18, -0.26) * scale,
        0.07 * scale,
        None,
        Color::new(0.93, 0.80, 0.62, 1.0),
    );
    draw_face(center, 0.22, -0.28, 0.07, scale);
}
