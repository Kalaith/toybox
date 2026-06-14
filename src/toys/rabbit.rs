use super::library::{brighten, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let inner = Color::new(0.96, 0.72, 0.78, 1.0);
    draw_sphere(center, 0.23 * scale, None, color);
    draw_sphere(
        center + vec3(0.0, -0.02, -0.16) * scale,
        0.12 * scale,
        None,
        Color::new(0.94, 0.90, 0.82, 1.0),
    );
    draw_sphere(
        center + vec3(0.0, 0.22, -0.17) * scale,
        0.15 * scale,
        None,
        brighten(color, 0.06),
    );
    draw_cube(
        center + vec3(-0.08, 0.45, -0.17) * scale,
        vec3(0.07, 0.28, 0.06) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.08, 0.45, -0.17) * scale,
        vec3(0.07, 0.28, 0.06) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(-0.08, 0.45, -0.205) * scale,
        vec3(0.035, 0.22, 0.018) * scale,
        None,
        inner,
    );
    draw_cube(
        center + vec3(0.08, 0.45, -0.205) * scale,
        vec3(0.035, 0.22, 0.018) * scale,
        None,
        inner,
    );
    draw_sphere(
        center + vec3(0.0, 0.01, 0.24) * scale,
        0.07 * scale,
        None,
        Color::new(0.94, 0.90, 0.82, 1.0),
    );
    for x in [-0.13_f32, 0.13] {
        draw_sphere(
            center + vec3(x, -0.16, -0.11) * scale,
            0.055 * scale,
            None,
            brighten(color, 0.06),
        );
    }
    draw_face(center, 0.23, -0.31, 0.06, scale);
}
