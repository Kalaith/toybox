use super::library::{brighten, darken, draw_cube_with_edges, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Wide, low scuttler with pincer arms out the sides.
    draw_cube_with_edges(
        center + vec3(0.0, 0.12, 0.0) * scale,
        vec3(0.44, 0.16, 0.30) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.23, 0.0) * scale,
        vec3(0.28, 0.08, 0.22) * scale,
        brighten(color, 0.08),
    );

    for side in [-1.0_f32, 1.0] {
        // Pincers: arm out, then two claw halves.
        draw_cube_with_edges(
            center + vec3(side * 0.30, 0.14, -0.06) * scale,
            vec3(0.14, 0.07, 0.07) * scale,
            darken(color, 0.10),
        );
        draw_cube_with_edges(
            center + vec3(side * 0.40, 0.14, -0.14) * scale,
            vec3(0.05, 0.06, 0.12) * scale,
            darken(color, 0.16),
        );
        draw_cube_with_edges(
            center + vec3(side * 0.34, 0.14, -0.16) * scale,
            vec3(0.05, 0.06, 0.08) * scale,
            darken(color, 0.16),
        );
        // Three scuttling legs per side with silver foot tips.
        for z in [-0.10_f32, 0.02, 0.13] {
            draw_cube_with_edges(
                center + vec3(side * 0.21, 0.005, z) * scale,
                vec3(0.045, 0.11, 0.045) * scale,
                darken(color, 0.14),
            );
            draw_toy_sphere(
                center + vec3(side * 0.21, -0.055, z) * scale,
                0.026 * scale,
                None,
                Color::new(0.80, 0.84, 0.84, 1.0),
            );
        }
    }

    // Vent slats across the front plate.
    for y in [0.09_f32, 0.13] {
        draw_cube(
            center + vec3(0.0, y, -0.155) * scale,
            vec3(0.14, 0.022, 0.015) * scale,
            None,
            Color::new(0.10, 0.12, 0.14, 1.0),
        );
    }

    // Rivets on the shell corners.
    for x in [-0.09_f32, 0.09] {
        for z in [-0.06_f32, 0.06] {
            draw_toy_sphere(
                center + vec3(x, 0.275, z) * scale,
                0.016 * scale,
                None,
                Color::new(0.82, 0.84, 0.80, 1.0),
            );
        }
    }

    // Eye stalks.
    for side in [-1.0_f32, 1.0] {
        draw_cube_with_edges(
            center + vec3(side * 0.07, 0.32, -0.08) * scale,
            vec3(0.03, 0.10, 0.03) * scale,
            darken(color, 0.12),
        );
        draw_toy_sphere(
            center + vec3(side * 0.07, 0.39, -0.08) * scale,
            0.035 * scale,
            None,
            Color::new(0.95, 0.76, 0.30, 1.0),
        );
        // Pupil dot looking forward.
        draw_cube(
            center + vec3(side * 0.07, 0.39, -0.112) * scale,
            vec3(0.016, 0.016, 0.008) * scale,
            None,
            Color::new(0.08, 0.07, 0.07, 1.0),
        );
    }
}
