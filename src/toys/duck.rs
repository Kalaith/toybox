use super::library::draw_toy_sphere;
use super::library::{brighten, darken, draw_eye_pair};
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
    // Lower mandible, slightly open.
    draw_cube(
        center + vec3(0.0, 0.125, -0.395) * scale,
        vec3(0.16, 0.035, 0.10) * scale,
        None,
        darken(beak, 0.12),
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
    draw_eye_pair(center, 0.24, -0.37, 0.06, scale);

    // Rosy cheek dots.
    for x in [-0.115_f32, 0.115] {
        draw_toy_sphere(
            center + vec3(x, 0.17, -0.33) * scale,
            0.028 * scale,
            None,
            Color::new(0.96, 0.64, 0.64, 1.0),
        );
    }

    // Little sailor cap with a royal band.
    let cap = Color::new(0.94, 0.94, 0.90, 1.0);
    draw_cube(
        center + vec3(0.0, 0.325, -0.25) * scale,
        vec3(0.16, 0.025, 0.16) * scale,
        None,
        cap,
    );
    draw_cube(
        center + vec3(0.0, 0.36, -0.25) * scale,
        vec3(0.11, 0.055, 0.11) * scale,
        None,
        cap,
    );
    draw_cube(
        center + vec3(0.0, 0.339, -0.25) * scale,
        vec3(0.115, 0.016, 0.115) * scale,
        None,
        Color::new(0.26, 0.46, 0.86, 1.0),
    );

    // Feather groove on each wing.
    for x in [-0.245_f32, 0.245] {
        draw_cube(
            center + vec3(x, 0.05, 0.0) * scale,
            vec3(0.012, 0.025, 0.20) * scale,
            None,
            darken(color, 0.16),
        );
    }

    // Upturned tail feather.
    draw_cube(
        center + vec3(0.0, 0.115, 0.285) * scale,
        vec3(0.045, 0.10, 0.045) * scale,
        None,
        darken(color, 0.02),
    );
}
