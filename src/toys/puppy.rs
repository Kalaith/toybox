use super::library::{brighten, darken, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_sphere(center, 0.25 * scale, None, color);
    draw_sphere(
        center + vec3(0.0, -0.03, -0.15) * scale,
        0.12 * scale,
        None,
        Color::new(0.92, 0.78, 0.60, 1.0),
    );
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
    draw_cube(
        center + vec3(0.0, 0.07, -0.24) * scale,
        vec3(0.24, 0.045, 0.040) * scale,
        None,
        Color::new(0.80, 0.18, 0.16, 1.0),
    );
    draw_sphere(
        center + vec3(0.11, 0.08, -0.26) * scale,
        0.022 * scale,
        None,
        Color::new(0.96, 0.78, 0.28, 1.0),
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
    for x in [-0.14_f32, 0.14] {
        draw_sphere(
            center + vec3(x, -0.16, -0.12) * scale,
            0.055 * scale,
            None,
            brighten(color, 0.04),
        );
    }
    draw_face(center, 0.23, -0.32, 0.07, scale);
}
