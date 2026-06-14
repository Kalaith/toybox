use super::library::draw_game_box;
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);
    let stone = Color::new(0.82, 0.82, 0.78, 1.0);
    draw_cube(
        center + vec3(-0.15, 0.17, -0.02) * scale,
        vec3(0.09, 0.13, 0.09) * scale,
        None,
        stone,
    );
    draw_cube(
        center + vec3(0.15, 0.17, -0.02) * scale,
        vec3(0.09, 0.13, 0.09) * scale,
        None,
        stone,
    );
    draw_cube(
        center + vec3(0.0, 0.15, -0.02) * scale,
        vec3(0.13, 0.09, 0.09) * scale,
        None,
        stone,
    );
    draw_cube(
        center + vec3(0.0, 0.24, -0.02) * scale,
        vec3(0.07, 0.07, 0.07) * scale,
        None,
        Color::new(0.72, 0.22, 0.20, 1.0),
    );
    for x in [-0.15_f32, 0.15] {
        draw_cube(
            center + vec3(x, 0.25, -0.02) * scale,
            vec3(0.055, 0.050, 0.070) * scale,
            None,
            Color::new(0.72, 0.22, 0.20, 1.0),
        );
    }
}
