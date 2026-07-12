use super::library::{draw_studded_block, shift_block_color};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Stepped pyramid: three shrinking tiers and a cap stud.
    let tiers = [(0.0_f32, 0.40_f32), (0.15, 0.28), (0.30, 0.16)];
    for (index, (height, width)) in tiers.iter().enumerate() {
        draw_studded_block(
            center + vec3(0.0, -0.02 + height, 0.0) * scale,
            vec3(*width, 0.15, *width) * scale,
            shift_block_color(color, index),
        );
    }
    draw_studded_block(
        center + vec3(0.0, 0.43, 0.0) * scale,
        vec3(0.08, 0.10, 0.08) * scale,
        shift_block_color(color, 3),
    );
}
