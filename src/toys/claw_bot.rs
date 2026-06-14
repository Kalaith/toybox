use super::library::{darken, draw_cube_with_edges, draw_robot_core};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);
    draw_cube_with_edges(
        center + vec3(-0.28, 0.12, -0.09) * scale,
        vec3(0.08, 0.26, 0.08) * scale,
        darken(color, 0.12),
    );
    draw_cube_with_edges(
        center + vec3(0.28, 0.12, -0.09) * scale,
        vec3(0.08, 0.26, 0.08) * scale,
        darken(color, 0.12),
    );
    let metal = Color::new(0.90, 0.92, 0.92, 1.0);
    draw_cube(
        center + vec3(-0.34, -0.04, -0.12) * scale,
        vec3(0.16, 0.05, 0.05) * scale,
        None,
        metal,
    );
    draw_cube(
        center + vec3(0.34, -0.04, -0.12) * scale,
        vec3(0.16, 0.05, 0.05) * scale,
        None,
        metal,
    );
    for x in [-0.39_f32, 0.39] {
        draw_cube(
            center + vec3(x, -0.01, -0.16) * scale,
            vec3(0.045, 0.10, 0.045) * scale,
            None,
            metal,
        );
    }
}
