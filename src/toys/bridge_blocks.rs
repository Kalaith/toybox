use super::library::{
    brighten, darken, draw_cube_with_edges, draw_studded_block, shift_block_color,
};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_studded_block(
        center + vec3(0.0, 0.10, 0.0) * scale,
        vec3(0.58, 0.12, 0.22) * scale,
        color,
    );
    for (index, offset) in [-0.22_f32, 0.0, 0.22].into_iter().enumerate() {
        draw_studded_block(
            center + vec3(offset, -0.10, 0.0) * scale,
            vec3(0.10, 0.28, 0.18) * scale,
            shift_block_color(brighten(color, 0.08), index),
        );
    }

    // End towers with a spanning top beam, tower-bridge style.
    for x in [-0.22_f32, 0.22] {
        draw_studded_block(
            center + vec3(x, 0.26, 0.0) * scale,
            vec3(0.09, 0.20, 0.14) * scale,
            brighten(color, 0.14),
        );
    }
    draw_cube_with_edges(
        center + vec3(0.0, 0.345, 0.0) * scale,
        vec3(0.36, 0.035, 0.05) * scale,
        darken(color, 0.10),
    );

    // Low side rails along the deck edges.
    for z in [-0.10_f32, 0.10] {
        draw_cube_with_edges(
            center + vec3(0.0, 0.185, z) * scale,
            vec3(0.42, 0.030, 0.022) * scale,
            darken(color, 0.06),
        );
    }

    // Ramp blocks stepping down at both ends.
    for x in [-0.335_f32, 0.335] {
        draw_studded_block(
            center + vec3(x, 0.045, 0.0) * scale,
            vec3(0.11, 0.07, 0.20) * scale,
            darken(color, 0.08),
        );
    }
}
