use super::library::draw_toy_sphere;
use super::library::{brighten, darken, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_toy_sphere(center, 0.24 * scale, None, color);
    draw_toy_sphere(
        center + vec3(0.0, -0.02, -0.15) * scale,
        0.11 * scale,
        None,
        brighten(color, 0.08),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.22, -0.16) * scale,
        0.16 * scale,
        None,
        brighten(color, 0.04),
    );
    draw_cube(
        center + vec3(-0.12, 0.39, -0.16) * scale,
        vec3(0.10, 0.12, 0.08) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.12, 0.39, -0.16) * scale,
        vec3(0.10, 0.12, 0.08) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(-0.12, 0.40, -0.20) * scale,
        vec3(0.045, 0.070, 0.020) * scale,
        None,
        Color::new(0.96, 0.66, 0.74, 1.0),
    );
    draw_cube(
        center + vec3(0.12, 0.40, -0.20) * scale,
        vec3(0.045, 0.070, 0.020) * scale,
        None,
        Color::new(0.96, 0.66, 0.74, 1.0),
    );
    // Tail curling up and forward in three segments.
    draw_cube(
        center + vec3(0.24, 0.02, 0.18) * scale,
        vec3(0.075, 0.20, 0.075) * scale,
        None,
        darken(color, 0.08),
    );
    draw_cube(
        center + vec3(0.24, 0.16, 0.155) * scale,
        vec3(0.062, 0.16, 0.062) * scale,
        None,
        darken(color, 0.03),
    );
    draw_toy_sphere(
        center + vec3(0.24, 0.26, 0.125) * scale,
        0.048 * scale,
        None,
        brighten(color, 0.14),
    );
    draw_cube(
        center + vec3(0.0, 0.18, -0.31) * scale,
        vec3(0.05, 0.04, 0.05) * scale,
        None,
        Color::new(0.06, 0.04, 0.04, 1.0),
    );
    for y in [0.18_f32, 0.22] {
        draw_cube(
            center + vec3(-0.12, y, -0.34) * scale,
            vec3(0.11, 0.010, 0.010) * scale,
            None,
            Color::new(0.93, 0.88, 0.74, 1.0),
        );
        draw_cube(
            center + vec3(0.12, y, -0.34) * scale,
            vec3(0.11, 0.010, 0.010) * scale,
            None,
            Color::new(0.93, 0.88, 0.74, 1.0),
        );
    }
    draw_face(center, 0.25, -0.31, 0.07, scale);

    // Front paws peeking out under the chest.
    for x in [-0.10_f32, 0.10] {
        draw_toy_sphere(
            center + vec3(x, -0.20, -0.14) * scale,
            0.06 * scale,
            None,
            brighten(color, 0.06),
        );
    }

    // Tabby stripes across the back.
    for x in [-0.07_f32, 0.0, 0.07] {
        draw_cube(
            center + vec3(x, 0.225, 0.02) * scale,
            vec3(0.030, 0.016, 0.15) * scale,
            None,
            darken(color, 0.14),
        );
    }

    // Cherry collar with a gold bell at the throat.
    for x in [-0.075_f32, 0.075] {
        draw_cube(
            center + vec3(x, 0.075, -0.21) * scale,
            vec3(0.055, 0.028, 0.030) * scale,
            None,
            Color::new(0.72, 0.20, 0.22, 1.0),
        );
    }
    draw_toy_sphere(
        center + vec3(0.0, 0.055, -0.25) * scale,
        0.032 * scale,
        None,
        Color::new(0.95, 0.78, 0.30, 1.0),
    );
}
