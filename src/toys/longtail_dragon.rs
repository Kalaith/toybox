use super::library::draw_toy_sphere;
use super::library::{darken, draw_dragon_base};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);
    for index in 0..4 {
        draw_toy_sphere(
            center + vec3(0.0, 0.02 - index as f32 * 0.01, 0.27 + index as f32 * 0.14) * scale,
            (0.09 - index as f32 * 0.012) * scale,
            None,
            darken(color, 0.05),
        );
    }
    draw_toy_sphere(
        center + vec3(0.0, -0.02, 0.82) * scale,
        0.040 * scale,
        None,
        Color::new(0.96, 0.84, 0.36, 1.0),
    );
}
