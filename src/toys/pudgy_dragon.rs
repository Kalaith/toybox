use super::library::{brighten, darken, draw_eye_pair, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // One big round ball of dragon with comically small wings.
    draw_toy_sphere(
        center + vec3(0.0, 0.10, 0.0) * scale,
        0.26 * scale,
        None,
        color,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.04, -0.16) * scale,
        0.16 * scale,
        None,
        Color::new(0.90, 0.76, 0.54, 1.0),
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.26, 0.24, 0.06) * scale,
            vec3(0.05, 0.10, 0.14) * scale,
            None,
            darken(color, 0.12),
        );
        draw_cube(
            center + vec3(side * 0.12, -0.12, 0.02) * scale,
            vec3(0.09, 0.07, 0.09) * scale,
            None,
            darken(color, 0.06),
        );
    }

    // Snout stub, tiny horns, stubby tail.
    draw_toy_sphere(
        center + vec3(0.0, 0.16, -0.24) * scale,
        0.09 * scale,
        None,
        brighten(color, 0.10),
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.08, 0.36, -0.04) * scale,
            vec3(0.04, 0.09, 0.04) * scale,
            None,
            Color::new(0.96, 0.88, 0.58, 1.0),
        );
    }
    draw_toy_sphere(
        center + vec3(0.0, 0.06, 0.27) * scale,
        0.07 * scale,
        None,
        brighten(color, 0.08),
    );
    draw_eye_pair(center, 0.22, -0.30, 0.07, scale);
}
