use super::library::draw_toy_sphere;
use super::library::{brighten, draw_cube_with_edges, draw_dragon_base};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);
    let horn = Color::new(0.96, 0.88, 0.58, 1.0);
    draw_cube_with_edges(
        center + vec3(-0.10, 0.36, -0.34) * scale,
        vec3(0.06, 0.18, 0.06) * scale,
        horn,
    );
    draw_cube_with_edges(
        center + vec3(0.10, 0.36, -0.34) * scale,
        vec3(0.06, 0.18, 0.06) * scale,
        horn,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.05, 0.34) * scale,
        0.08 * scale,
        None,
        brighten(color, 0.12),
    );
    draw_toy_sphere(
        center + vec3(0.06, 0.08, 0.42) * scale,
        0.055 * scale,
        None,
        horn,
    );
}
