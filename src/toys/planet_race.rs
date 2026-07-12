use super::library::draw_game_box;
use super::library::draw_toy_sphere;
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);
    draw_toy_sphere(
        center + vec3(-0.12, 0.15, -0.04) * scale,
        0.07 * scale,
        None,
        Color::new(0.40, 0.70, 0.96, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.08, 0.15, -0.03) * scale,
        0.05 * scale,
        None,
        Color::new(0.96, 0.66, 0.30, 1.0),
    );
    draw_cube(
        center + vec3(0.18, 0.13, 0.05) * scale,
        vec3(0.11, 0.03, 0.04) * scale,
        None,
        Color::new(0.94, 0.94, 0.88, 1.0),
    );
    draw_cube(
        center + vec3(-0.01, 0.145, -0.03) * scale,
        vec3(0.27, 0.012, 0.020) * scale,
        None,
        Color::new(0.88, 0.92, 1.0, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.20, 0.155, -0.07) * scale,
        0.024 * scale,
        None,
        Color::new(0.96, 0.90, 0.46, 1.0),
    );
}
