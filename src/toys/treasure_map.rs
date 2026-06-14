use super::library::draw_game_box;
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);
    draw_cube(
        center + vec3(0.0, 0.14, -0.02) * scale,
        vec3(0.34, 0.03, 0.22) * scale,
        None,
        Color::new(0.92, 0.78, 0.50, 1.0),
    );
    draw_cube(
        center + vec3(-0.09, 0.17, -0.02) * scale,
        vec3(0.05, 0.035, 0.05) * scale,
        None,
        Color::new(0.76, 0.18, 0.14, 1.0),
    );
    draw_cube(
        center + vec3(0.12, 0.17, 0.04) * scale,
        vec3(0.08, 0.025, 0.08) * scale,
        None,
        Color::new(0.08, 0.10, 0.10, 1.0),
    );
    draw_cube(
        center + vec3(0.03, 0.171, -0.05) * scale,
        vec3(0.16, 0.014, 0.018) * scale,
        None,
        Color::new(0.52, 0.34, 0.18, 1.0),
    );
    draw_cube(
        center + vec3(0.02, 0.173, 0.04) * scale,
        vec3(0.045, 0.014, 0.045) * scale,
        None,
        Color::new(0.92, 0.64, 0.18, 1.0),
    );
}
