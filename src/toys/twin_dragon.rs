use super::library::{brighten, darken, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Shared body with two necks and two heads side by side.
    draw_toy_sphere(
        center + vec3(0.0, 0.08, 0.06) * scale,
        0.22 * scale,
        None,
        color,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.05, 0.30) * scale,
        0.09 * scale,
        None,
        brighten(color, 0.08),
    );

    for side in [-1.0_f32, 1.0] {
        let neck_base = center + vec3(side * 0.10, 0.20, -0.10) * scale;
        draw_cube(
            neck_base,
            vec3(0.07, 0.18, 0.07) * scale,
            None,
            darken(color, 0.05),
        );
        let head = center + vec3(side * 0.13, 0.34, -0.16) * scale;
        draw_toy_sphere(
            head,
            0.10 * scale,
            None,
            if side < 0.0 {
                brighten(color, 0.10)
            } else {
                brighten(color, 0.02)
            },
        );
        draw_cube(
            head + vec3(0.0, 0.11, 0.0) * scale,
            vec3(0.035, 0.08, 0.035) * scale,
            None,
            Color::new(0.96, 0.88, 0.58, 1.0),
        );
        draw_toy_sphere(
            head + vec3(0.0, 0.01, -0.095) * scale,
            0.025 * scale,
            None,
            Color::new(0.07, 0.06, 0.05, 1.0),
        );
    }

    for x in [-0.12_f32, 0.12] {
        draw_cube(
            center + vec3(x, -0.10, 0.0) * scale,
            vec3(0.08, 0.08, 0.08) * scale,
            None,
            darken(color, 0.08),
        );
    }
}
