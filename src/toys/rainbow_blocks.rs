use super::library::{draw_studded_block, shift_block_color};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Five blocks stacked into an arch: two columns and a bridge over the top.
    let arc = [
        (vec3(-0.22, 0.02, 0.0), vec3(0.14, 0.20, 0.20)),
        (vec3(0.22, 0.02, 0.0), vec3(0.14, 0.20, 0.20)),
        (vec3(-0.19, 0.20, 0.0), vec3(0.13, 0.16, 0.18)),
        (vec3(0.19, 0.20, 0.0), vec3(0.13, 0.16, 0.18)),
        (vec3(0.0, 0.33, 0.0), vec3(0.42, 0.13, 0.16)),
    ];
    for (index, (offset, size)) in arc.iter().enumerate() {
        draw_studded_block(
            center + *offset * scale,
            *size * scale,
            shift_block_color(color, index),
        );
    }
}
