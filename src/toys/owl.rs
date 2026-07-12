use super::library::{brighten, darken, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Upright egg body with a pale belly panel.
    draw_toy_sphere(
        center + vec3(0.0, 0.10, 0.0) * scale,
        0.22 * scale,
        None,
        color,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.30, 0.0) * scale,
        0.17 * scale,
        None,
        brighten(color, 0.06),
    );
    draw_cube(
        center + vec3(0.0, 0.10, -0.17) * scale,
        vec3(0.19, 0.22, 0.06) * scale,
        None,
        brighten(color, 0.20),
    );

    // Folded wings and ear tufts.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.22, 0.10, 0.02) * scale,
            vec3(0.06, 0.26, 0.16) * scale,
            None,
            darken(color, 0.10),
        );
        draw_cube(
            center + vec3(side * 0.10, 0.47, 0.0) * scale,
            vec3(0.05, 0.10, 0.05) * scale,
            None,
            darken(color, 0.12),
        );
    }

    // Big round eyes with a small beak between them.
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.08, 0.33, -0.14) * scale,
            0.055 * scale,
            None,
            Color::new(0.96, 0.94, 0.88, 1.0),
        );
        draw_toy_sphere(
            center + vec3(side * 0.08, 0.33, -0.185) * scale,
            0.028 * scale,
            None,
            Color::new(0.08, 0.07, 0.06, 1.0),
        );
    }
    draw_cube(
        center + vec3(0.0, 0.26, -0.17) * scale,
        vec3(0.05, 0.06, 0.06) * scale,
        None,
        Color::new(0.94, 0.72, 0.30, 1.0),
    );
}
