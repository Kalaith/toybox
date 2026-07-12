use super::library::{brighten, darken, draw_cube_with_edges};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // One oversized chunky cube of a robot with stubby limbs.
    draw_cube_with_edges(
        center + vec3(0.0, 0.22, 0.0) * scale,
        vec3(0.40, 0.40, 0.34) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.26, -0.175) * scale,
        vec3(0.28, 0.18, 0.04) * scale,
        darken(color, 0.20),
    );
    for side in [-1.0_f32, 1.0] {
        // Square eyes on the face panel.
        draw_cube_with_edges(
            center + vec3(side * 0.08, 0.30, -0.20) * scale,
            vec3(0.06, 0.06, 0.02) * scale,
            Color::new(0.95, 0.86, 0.40, 1.0),
        );
        // Blocky arms and feet.
        draw_cube_with_edges(
            center + vec3(side * 0.26, 0.18, 0.0) * scale,
            vec3(0.10, 0.24, 0.12) * scale,
            darken(color, 0.10),
        );
        draw_cube_with_edges(
            center + vec3(side * 0.11, -0.04, 0.0) * scale,
            vec3(0.13, 0.12, 0.18) * scale,
            darken(color, 0.14),
        );
    }
    // Rivet strip across the top.
    draw_cube_with_edges(
        center + vec3(0.0, 0.44, 0.0) * scale,
        vec3(0.20, 0.05, 0.14) * scale,
        brighten(color, 0.10),
    );
}
