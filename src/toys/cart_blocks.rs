use super::library::{draw_studded_block, draw_wheel, shift_block_color};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let wheel = Color::new(0.22, 0.22, 0.24, 1.0);

    // Rolling block cart: flat chassis on four wheels, cargo blocks on top.
    draw_studded_block(
        center + vec3(0.0, 0.10, 0.0) * scale,
        vec3(0.42, 0.10, 0.26) * scale,
        color,
    );
    for x in [-0.15_f32, 0.15] {
        for z in [-0.14_f32, 0.14] {
            draw_wheel(
                center + vec3(x, 0.02, z) * scale,
                0.055 * scale,
                0.04 * scale,
                wheel,
            );
        }
    }

    draw_studded_block(
        center + vec3(-0.09, 0.22, 0.0) * scale,
        vec3(0.16, 0.14, 0.18) * scale,
        shift_block_color(color, 1),
    );
    draw_studded_block(
        center + vec3(0.12, 0.20, 0.0) * scale,
        vec3(0.12, 0.10, 0.14) * scale,
        shift_block_color(color, 2),
    );
    draw_studded_block(
        center + vec3(-0.09, 0.33, 0.0) * scale,
        vec3(0.10, 0.09, 0.12) * scale,
        shift_block_color(color, 3),
    );
}
