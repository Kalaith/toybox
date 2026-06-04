use super::library::{brighten, darken};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_cube(
        center + vec3(0.0, -0.08, 0.0) * scale,
        vec3(0.44, 0.18, 0.24) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(-0.18, 0.12, 0.0) * scale,
        vec3(0.14, 0.28, 0.20) * scale,
        None,
        brighten(color, 0.08),
    );
    draw_cube(
        center + vec3(0.18, 0.12, 0.0) * scale,
        vec3(0.14, 0.28, 0.20) * scale,
        None,
        brighten(color, 0.08),
    );
    draw_cube(
        center + vec3(0.0, 0.18, 0.0) * scale,
        vec3(0.16, 0.16, 0.20) * scale,
        None,
        darken(color, 0.10),
    );
}
