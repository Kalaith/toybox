use super::library::shift_block_color;
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    for index in 0..4 {
        draw_cube(
            center + vec3(0.0, -0.09 + index as f32 * 0.16, 0.0) * scale,
            vec3(0.24, 0.16, 0.24) * scale,
            None,
            shift_block_color(color, index),
        );
    }
}
