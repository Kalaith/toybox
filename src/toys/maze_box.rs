use super::library::draw_game_box;
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);
    let ink = Color::new(0.08, 0.09, 0.10, 1.0);
    for offset in [-0.16_f32, 0.0, 0.16] {
        draw_cube(
            center + vec3(offset, 0.12, -0.05) * scale,
            vec3(0.035, 0.035, 0.24) * scale,
            None,
            ink,
        );
        draw_cube(
            center + vec3(0.02, 0.13, offset * 0.7) * scale,
            vec3(0.30, 0.032, 0.03) * scale,
            None,
            ink,
        );
    }
    draw_sphere(
        center + vec3(0.16, 0.155, 0.08) * scale,
        0.035 * scale,
        None,
        Color::new(0.94, 0.72, 0.24, 1.0),
    );
}
