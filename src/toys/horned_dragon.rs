use super::library::{darken, draw_cube_with_edges, draw_dragon_base};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);
    let horn = Color::new(0.96, 0.88, 0.58, 1.0);
    draw_cube_with_edges(
        center + vec3(-0.16, 0.28, -0.35) * scale,
        vec3(0.07, 0.24, 0.06) * scale,
        horn,
    );
    draw_cube_with_edges(
        center + vec3(0.16, 0.28, -0.35) * scale,
        vec3(0.07, 0.24, 0.06) * scale,
        horn,
    );
    draw_cube(
        center + vec3(0.0, 0.13, -0.45) * scale,
        vec3(0.12, 0.06, 0.14) * scale,
        None,
        darken(color, 0.06),
    );
    for x in [-0.04_f32, 0.04] {
        draw_cube(
            center + vec3(x, 0.15, -0.53) * scale,
            vec3(0.026, 0.022, 0.020) * scale,
            None,
            Color::new(0.04, 0.03, 0.025, 1.0),
        );
    }
}
