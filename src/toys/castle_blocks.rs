use super::library::{brighten, darken, draw_cube_with_edges, draw_studded_block};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_studded_block(
        center + vec3(0.0, -0.08, 0.0) * scale,
        vec3(0.44, 0.18, 0.24) * scale,
        color,
    );
    draw_studded_block(
        center + vec3(-0.18, 0.12, 0.0) * scale,
        vec3(0.14, 0.28, 0.20) * scale,
        brighten(color, 0.08),
    );
    draw_studded_block(
        center + vec3(0.18, 0.12, 0.0) * scale,
        vec3(0.14, 0.28, 0.20) * scale,
        brighten(color, 0.08),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.18, 0.0) * scale,
        vec3(0.16, 0.16, 0.20) * scale,
        darken(color, 0.10),
    );
}
