use super::primitives::{draw_studded_block, shift_block_color};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Stacked slightly askew, the way a kid actually builds one.
    let ink = Color::new(0.20, 0.17, 0.15, 1.0);
    let paper = Color::new(0.93, 0.90, 0.82, 1.0);
    let stack = [
        (0.0_f32, 0.0_f32, 0.24_f32),
        (0.03, -0.02, 0.23),
        (-0.025, 0.03, 0.22),
        (0.05, 0.02, 0.21),
    ];
    for (index, (dx, dz, width)) in stack.into_iter().enumerate() {
        let y = -0.09 + index as f32 * 0.16;
        draw_studded_block(
            center + vec3(dx, y, dz) * scale,
            vec3(width, 0.16, width) * scale,
            shift_block_color(color, index),
        );
        // Printed panel on the front face with a simple stamp.
        draw_cube(
            center + vec3(dx, y, dz - width * 0.5 - 0.005) * scale,
            vec3(0.10, 0.10, 0.012) * scale,
            None,
            paper,
        );
        let stamp = match index % 3 {
            0 => vec3(0.055, 0.055, 0.012),
            1 => vec3(0.075, 0.028, 0.012),
            _ => vec3(0.028, 0.075, 0.012),
        };
        draw_cube(
            center + vec3(dx, y, dz - width * 0.5 - 0.010) * scale,
            stamp * scale,
            None,
            ink,
        );
    }

    // Spare blocks waiting at the base.
    draw_studded_block(
        center + vec3(0.25, -0.12, -0.10) * scale,
        vec3(0.14, 0.10, 0.14) * scale,
        shift_block_color(color, 4),
    );
    draw_studded_block(
        center + vec3(-0.23, -0.135, 0.08) * scale,
        vec3(0.12, 0.08, 0.12) * scale,
        shift_block_color(color, 5),
    );
}
