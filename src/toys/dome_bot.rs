use super::library::{brighten, darken, draw_cube_with_edges, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Retro vacuum-bot: squat drum body under a glass-dome head.
    draw_cube_with_edges(
        center + vec3(0.0, 0.10, 0.0) * scale,
        vec3(0.40, 0.22, 0.36) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.24, 0.0) * scale,
        vec3(0.34, 0.06, 0.30) * scale,
        darken(color, 0.12),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.34, 0.0) * scale,
        0.15 * scale,
        None,
        Color::new(0.72, 0.86, 0.92, 0.96),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.32, -0.06) * scale,
        0.05 * scale,
        None,
        Color::new(0.95, 0.76, 0.30, 1.0),
    );

    // Bumper skirt and little side sensors.
    draw_cube_with_edges(
        center + vec3(0.0, -0.02, 0.0) * scale,
        vec3(0.46, 0.08, 0.42) * scale,
        darken(color, 0.18),
    );
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.24, 0.16, -0.12) * scale,
            0.035 * scale,
            None,
            Color::new(0.92, 0.30, 0.24, 1.0),
        );
    }
    draw_cube_with_edges(
        center + vec3(0.0, 0.12, -0.20) * scale,
        vec3(0.14, 0.05, 0.05) * scale,
        brighten(color, 0.12),
    );
}
