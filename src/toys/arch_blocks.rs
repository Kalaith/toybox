use super::library::{brighten, darken, draw_cube_with_edges, draw_studded_block};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_studded_block(
        center + vec3(-0.16, -0.02, 0.0) * scale,
        vec3(0.16, 0.34, 0.20) * scale,
        color,
    );
    draw_studded_block(
        center + vec3(0.16, -0.02, 0.0) * scale,
        vec3(0.16, 0.34, 0.20) * scale,
        brighten(color, 0.10),
    );

    // Corbel steps narrowing the opening so the span reads as an arch
    // rather than a flat doorway.
    draw_cube_with_edges(
        center + vec3(-0.065, 0.105, 0.0) * scale,
        vec3(0.055, 0.08, 0.18) * scale,
        darken(color, 0.04),
    );
    draw_cube_with_edges(
        center + vec3(0.065, 0.105, 0.0) * scale,
        vec3(0.055, 0.08, 0.18) * scale,
        brighten(color, 0.05),
    );

    draw_studded_block(
        center + vec3(0.0, 0.22, 0.0) * scale,
        vec3(0.46, 0.16, 0.20) * scale,
        darken(color, 0.08),
    );

    // Keystone sitting proud of the lintel.
    draw_studded_block(
        center + vec3(0.0, 0.345, 0.0) * scale,
        vec3(0.14, 0.09, 0.22) * scale,
        brighten(color, 0.18),
    );

    // Peg mast and pennant, the classic castle-set topper.
    draw_cube_with_edges(
        center + vec3(0.0, 0.47, 0.0) * scale,
        vec3(0.018, 0.15, 0.018) * scale,
        Color::new(0.42, 0.30, 0.22, 1.0),
    );
    draw_cube(
        center + vec3(0.055, 0.515, 0.0) * scale,
        vec3(0.09, 0.045, 0.014) * scale,
        None,
        Color::new(0.96, 0.78, 0.26, 1.0),
    );
}
