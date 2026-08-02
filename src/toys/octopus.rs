use super::primitives::{brighten, darken, draw_eye_pair, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Big dome head sitting on a ring of stubby tentacles.
    draw_toy_sphere(
        center + vec3(0.0, 0.16, 0.0) * scale,
        0.21 * scale,
        None,
        color,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.30, 0.0) * scale,
        0.13 * scale,
        None,
        brighten(color, 0.06),
    );

    for index in 0..8 {
        let angle = index as f32 / 8.0 * std::f32::consts::TAU + 0.39;
        let radius = 0.17;
        let tip = 0.25;
        draw_cube(
            center + vec3(angle.cos() * radius, -0.02, angle.sin() * radius) * scale,
            vec3(0.08, 0.10, 0.08) * scale,
            None,
            darken(color, 0.08),
        );
        draw_cube(
            center + vec3(angle.cos() * tip, -0.07, angle.sin() * tip) * scale,
            vec3(0.06, 0.06, 0.06) * scale,
            None,
            darken(color, 0.14),
        );
        // Tip segment curling back up.
        let curl = 0.315;
        draw_cube(
            center + vec3(angle.cos() * curl, -0.035, angle.sin() * curl) * scale,
            vec3(0.045, 0.045, 0.045) * scale,
            None,
            brighten(color, 0.04),
        );
        // Sucker dot on alternating curled tips.
        if index % 2 == 0 {
            draw_toy_sphere(
                center + vec3(angle.cos() * curl, -0.005, angle.sin() * curl) * scale,
                0.015 * scale,
                None,
                Color::new(0.94, 0.90, 0.80, 1.0),
            );
        }
    }

    // Pale mottled spots across the dome.
    let spots = [
        (0.10_f32, 0.28, -0.09),
        (-0.13, 0.26, 0.02),
        (0.03, 0.33, 0.10),
    ];
    for (x, y, z) in spots {
        draw_toy_sphere(
            center + vec3(x, y, z) * scale,
            0.026 * scale,
            None,
            brighten(color, 0.16),
        );
    }

    draw_eye_pair(center, 0.20, -0.19, 0.08, scale);

    // Blush and a little smile.
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.13, 0.14, -0.16) * scale,
            0.025 * scale,
            None,
            Color::new(0.96, 0.64, 0.64, 1.0),
        );
    }
    draw_cube(
        center + vec3(0.0, 0.12, -0.205) * scale,
        vec3(0.045, 0.014, 0.012) * scale,
        None,
        Color::new(0.035, 0.030, 0.026, 1.0),
    );
}
