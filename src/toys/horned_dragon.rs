use super::library::{darken, draw_dragon_base};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);
    let horn = Color::new(0.96, 0.88, 0.58, 1.0);
    draw_cube(
        center + vec3(-0.16, 0.28, -0.35) * scale,
        vec3(0.07, 0.24, 0.06) * scale,
        None,
        horn,
    );
    draw_cube(
        center + vec3(0.16, 0.28, -0.35) * scale,
        vec3(0.07, 0.24, 0.06) * scale,
        None,
        horn,
    );
    draw_cube(
        center + vec3(0.0, 0.13, -0.45) * scale,
        vec3(0.12, 0.06, 0.14) * scale,
        None,
        darken(color, 0.06),
    );
}
