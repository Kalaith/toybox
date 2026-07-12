use super::library::{brighten, darken, draw_cube_with_edges, draw_robot_arms, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Slim torso balancing on a single ball wheel.
    draw_toy_sphere(
        center + vec3(0.0, 0.02, 0.0) * scale,
        0.14 * scale,
        None,
        Color::new(0.24, 0.26, 0.28, 1.0),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.20, 0.0) * scale,
        vec3(0.16, 0.14, 0.14) * scale,
        darken(color, 0.10),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.36, 0.0) * scale,
        vec3(0.24, 0.22, 0.20) * scale,
        color,
    );
    draw_robot_arms(center + vec3(0.0, 0.10, 0.0) * scale, color, scale * 0.8);

    // Narrow visor head.
    draw_cube_with_edges(
        center + vec3(0.0, 0.54, 0.0) * scale,
        vec3(0.18, 0.10, 0.16) * scale,
        brighten(color, 0.08),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.55, -0.085) * scale,
        vec3(0.13, 0.04, 0.02) * scale,
        Color::new(0.42, 0.95, 0.96, 1.0),
    );
}
