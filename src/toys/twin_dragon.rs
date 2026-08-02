use super::primitives::{brighten, darken, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Shared body with two necks and two heads side by side.
    draw_toy_sphere(
        center + vec3(0.0, 0.08, 0.06) * scale,
        0.22 * scale,
        None,
        color,
    );
    // Tan belly patch.
    draw_toy_sphere(
        center + vec3(0.0, 0.02, -0.11) * scale,
        0.12 * scale,
        None,
        Color::new(0.90, 0.76, 0.54, 1.0),
    );
    // Forked twin tail, one prong per head.
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.09, 0.05, 0.29) * scale,
            0.065 * scale,
            None,
            brighten(color, 0.08),
        );
        draw_toy_sphere(
            center + vec3(side * 0.13, 0.09, 0.37) * scale,
            0.032 * scale,
            None,
            Color::new(0.96, 0.88, 0.58, 1.0),
        );
    }
    // Wing nubs on the shoulders.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.22, 0.18, 0.10) * scale,
            vec3(0.04, 0.08, 0.12) * scale,
            None,
            darken(color, 0.10),
        );
    }

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
        // Snout on each head.
        draw_toy_sphere(
            head + vec3(0.0, -0.02, -0.085) * scale,
            0.045 * scale,
            None,
            brighten(color, 0.16),
        );
        // Twins with personality: the left head is awake, the right
        // one dozes with shut-line eyes.
        let ink = Color::new(0.07, 0.06, 0.05, 1.0);
        for eye in [-1.0_f32, 1.0] {
            if side < 0.0 {
                draw_toy_sphere(
                    head + vec3(eye * 0.040, 0.030, -0.088) * scale,
                    0.018 * scale,
                    None,
                    ink,
                );
            } else {
                draw_cube(
                    head + vec3(eye * 0.040, 0.030, -0.094) * scale,
                    vec3(0.032, 0.010, 0.012) * scale,
                    None,
                    ink,
                );
            }
        }
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
