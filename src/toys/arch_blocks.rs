use super::library::{brighten, darken};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_cube(
        center + vec3(-0.16, -0.02, 0.0) * scale,
        vec3(0.16, 0.34, 0.20) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.16, -0.02, 0.0) * scale,
        vec3(0.16, 0.34, 0.20) * scale,
        None,
        brighten(color, 0.10),
    );
    draw_cube(
        center + vec3(0.0, 0.22, 0.0) * scale,
        vec3(0.46, 0.16, 0.20) * scale,
        None,
        darken(color, 0.08),
    );
}
