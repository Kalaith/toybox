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
}
