use super::library::{draw_studded_block, shift_block_color};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Helix tower: each block steps up and rotates around the axis.
    for index in 0..6 {
        let angle = index as f32 * 0.9;
        let radius = 0.13;
        draw_studded_block(
            center
                + vec3(
                    angle.cos() * radius,
                    -0.04 + index as f32 * 0.11,
                    angle.sin() * radius,
                ) * scale,
            vec3(0.16, 0.11, 0.16) * scale,
            shift_block_color(color, index),
        );
    }
}
