use super::library::draw_toy_sphere;
use super::library::{brighten, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let tan = Color::new(0.93, 0.80, 0.62, 1.0);
    draw_toy_sphere(center, 0.26 * scale, None, color);
    draw_toy_sphere(
        center + vec3(0.0, -0.04, -0.18) * scale,
        0.13 * scale,
        None,
        tan,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.22, -0.12) * scale,
        0.17 * scale,
        None,
        brighten(color, 0.06),
    );
    draw_toy_sphere(
        center + vec3(-0.16, 0.36, -0.12) * scale,
        0.08 * scale,
        None,
        color,
    );
    draw_toy_sphere(
        center + vec3(0.16, 0.36, -0.12) * scale,
        0.08 * scale,
        None,
        color,
    );
    for x in [-0.16_f32, 0.16] {
        draw_toy_sphere(
            center + vec3(x, 0.36, -0.15) * scale,
            0.045 * scale,
            None,
            tan,
        );
        draw_toy_sphere(
            center + vec3(x, -0.16, -0.13) * scale,
            0.065 * scale,
            None,
            brighten(color, 0.05),
        );
    }
    draw_toy_sphere(
        center + vec3(0.0, 0.18, -0.26) * scale,
        0.07 * scale,
        None,
        tan,
    );
    draw_face(center, 0.22, -0.28, 0.07, scale);
}
