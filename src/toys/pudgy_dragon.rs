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

    // Blush, nostrils, and one snaggle fang.
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.145, 0.14, -0.20) * scale,
            0.028 * scale,
            None,
            Color::new(0.96, 0.64, 0.64, 1.0),
        );
        draw_cube(
            center + vec3(side * 0.03, 0.185, -0.325) * scale,
            vec3(0.018, 0.018, 0.010) * scale,
            None,
            Color::new(0.06, 0.05, 0.04, 1.0),
        );
    }
    draw_cube(
        center + vec3(-0.045, 0.095, -0.315) * scale,
        vec3(0.018, 0.030, 0.012) * scale,
        None,
        Color::new(0.96, 0.94, 0.86, 1.0),
    );

    // Belly button on the tummy patch.
    draw_cube(
        center + vec3(0.0, 0.01, -0.312) * scale,
        vec3(0.018, 0.018, 0.010) * scale,
        None,
        Color::new(0.74, 0.60, 0.40, 1.0),
    );

    // Brighter membrane accents on the tiny wings.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.27, 0.24, 0.04) * scale,
            vec3(0.030, 0.070, 0.10) * scale,
            None,
            brighten(color, 0.10),
        );
    }

    // Chocolate-chip cookie clutched in a paw.
    draw_cube(
        center + vec3(0.155, 0.06, -0.20) * scale,
        vec3(0.075, 0.016, 0.075) * scale,
        None,
        Color::new(0.82, 0.62, 0.38, 1.0),
    );
    let chips = [(0.135_f32, -0.215_f32), (0.170, -0.185), (0.150, -0.225)];
    for (x, z) in chips {
        draw_cube(
            center + vec3(x, 0.072, z) * scale,
            vec3(0.014, 0.010, 0.014) * scale,
            None,
            Color::new(0.30, 0.20, 0.12, 1.0),
        );
    }
    draw_toy_sphere(
        center + vec3(0.175, 0.095, -0.185) * scale,
        0.045 * scale,
        None,
        color,
    );
}
