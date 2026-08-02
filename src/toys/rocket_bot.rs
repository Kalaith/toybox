use super::primitives::{brighten, darken, draw_cube_with_edges, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Tall rocket-shaped robot with landing fins and a nose dome.
    draw_cube_with_edges(
        center + vec3(0.0, 0.16, 0.0) * scale,
        vec3(0.20, 0.30, 0.20) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.38, 0.0) * scale,
        vec3(0.16, 0.16, 0.16) * scale,
        brighten(color, 0.08),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.52, 0.0) * scale,
        0.09 * scale,
        None,
        Color::new(0.90, 0.86, 0.80, 1.0),
    );

    // Porthole eye in a silver rim, and three landing fins.
    draw_toy_sphere(
        center + vec3(0.0, 0.38, -0.082) * scale,
        0.055 * scale,
        None,
        Color::new(0.80, 0.84, 0.84, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.38, -0.09) * scale,
        0.045 * scale,
        None,
        Color::new(0.42, 0.95, 0.96, 1.0),
    );

    // Fold-out arms with mitten hands.
    for side in [-1.0_f32, 1.0] {
        draw_cube_with_edges(
            center + vec3(side * 0.125, 0.20, 0.0) * scale,
            vec3(0.045, 0.16, 0.045) * scale,
            darken(color, 0.10),
        );
        draw_toy_sphere(
            center + vec3(side * 0.125, 0.095, 0.0) * scale,
            0.038 * scale,
            None,
            Color::new(0.80, 0.84, 0.84, 1.0),
        );
    }

    // Racing stripe down the front.
    draw_cube(
        center + vec3(0.0, 0.16, -0.105) * scale,
        vec3(0.05, 0.28, 0.012) * scale,
        None,
        Color::new(0.94, 0.90, 0.80, 1.0),
    );
    for index in 0..3 {
        let angle = index as f32 / 3.0 * std::f32::consts::TAU + 0.52;
        draw_cube_with_edges(
            center + vec3(angle.cos() * 0.15, 0.02, angle.sin() * 0.15) * scale,
            vec3(0.06, 0.18, 0.06) * scale,
            darken(color, 0.14),
        );
    }
    // Thruster bell glow with a fading exhaust flame below.
    draw_toy_sphere(
        center + vec3(0.0, -0.02, 0.0) * scale,
        0.07 * scale,
        None,
        Color::new(0.95, 0.58, 0.24, 1.0),
    );
    let flame = [
        (-0.10_f32, 0.048_f32, 0.90_f32),
        (-0.155, 0.034, 0.55),
        (-0.195, 0.022, 0.30),
    ];
    for (y, radius, alpha) in flame {
        draw_toy_sphere(
            center + vec3(0.0, y, 0.0) * scale,
            radius * scale,
            None,
            Color::new(0.97, 0.80, 0.34, alpha),
        );
    }
}
