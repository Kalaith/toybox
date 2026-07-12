use super::library::{darken, draw_studded_block, shift_block_color};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Little block cottage: walls, stepped roof ridge, chimney, door tile.
    draw_studded_block(
        center + vec3(0.0, 0.06, 0.0) * scale,
        vec3(0.36, 0.26, 0.30) * scale,
        color,
    );
    draw_studded_block(
        center + vec3(0.0, 0.25, 0.0) * scale,
        vec3(0.40, 0.10, 0.34) * scale,
        shift_block_color(color, 1),
    );
    draw_studded_block(
        center + vec3(0.0, 0.34, 0.0) * scale,
        vec3(0.24, 0.09, 0.24) * scale,
        shift_block_color(color, 2),
    );
    draw_studded_block(
        center + vec3(0.0, 0.42, 0.0) * scale,
        vec3(0.10, 0.08, 0.14) * scale,
        shift_block_color(color, 3),
    );

    // Chimney and a dark doorway tile.
    draw_studded_block(
        center + vec3(0.13, 0.42, 0.08) * scale,
        vec3(0.06, 0.14, 0.06) * scale,
        shift_block_color(color, 4),
    );
    draw_cube(
        center + vec3(0.0, 0.02, -0.155) * scale,
        vec3(0.09, 0.16, 0.02) * scale,
        None,
        darken(color, 0.30),
    );
}
