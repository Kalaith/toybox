use super::library::{brighten, darken, draw_cube_with_edges};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let pip = Color::new(0.95, 0.94, 0.90, 1.0);

    // Tall dice-rolling tower with a chute mouth at the bottom.
    draw_cube_with_edges(
        center + vec3(-0.08, 0.24, 0.04) * scale,
        vec3(0.20, 0.46, 0.20) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(-0.08, 0.50, 0.04) * scale,
        vec3(0.24, 0.07, 0.24) * scale,
        darken(color, 0.12),
    );
    draw_cube_with_edges(
        center + vec3(-0.08, 0.06, -0.12) * scale,
        vec3(0.16, 0.10, 0.14) * scale,
        brighten(color, 0.10),
    );

    // Two dice spilled out front, pips face up.
    for (offset, die_color) in [
        (vec3(0.14, 0.05, -0.16), pip),
        (vec3(0.24, 0.045, 0.0), brighten(color, 0.25)),
    ] {
        draw_cube_with_edges(
            center + offset * scale,
            vec3(0.09, 0.09, 0.09) * scale,
            die_color,
        );
        draw_cube(
            center + (offset + vec3(0.0, 0.055, 0.0)) * scale,
            vec3(0.025, 0.01, 0.025) * scale,
            None,
            Color::new(0.10, 0.09, 0.08, 1.0),
        );
    }
}
