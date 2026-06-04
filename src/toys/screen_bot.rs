use super::library::{draw_robot_arms, draw_robot_core};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);
    draw_cube(
        center + vec3(0.0, 0.39, -0.15) * scale,
        vec3(0.21, 0.14, 0.035) * scale,
        None,
        Color::new(0.08, 0.13, 0.17, 1.0),
    );
    let glow = Color::new(0.46, 0.95, 0.92, 1.0);
    draw_cube(
        center + vec3(-0.05, 0.40, -0.18) * scale,
        vec3(0.04, 0.035, 0.025) * scale,
        None,
        glow,
    );
    draw_cube(
        center + vec3(0.05, 0.40, -0.18) * scale,
        vec3(0.04, 0.035, 0.025) * scale,
        None,
        glow,
    );
    draw_robot_arms(center, color, scale);
}
