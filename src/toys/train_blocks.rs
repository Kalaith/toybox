use super::library::{brighten, darken};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_cube(
        center + vec3(-0.10, 0.04, 0.0) * scale,
        vec3(0.42, 0.22, 0.22) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.18, 0.14, 0.0) * scale,
        vec3(0.18, 0.20, 0.20) * scale,
        None,
        brighten(color, 0.08),
    );
    let wheel_color = darken(color, 0.22);
    draw_cube(
        center + vec3(-0.06, -0.10, -0.10) * scale,
        vec3(0.54, 0.08, 0.08) * scale,
        None,
        wheel_color,
    );
    for offset in [-0.22_f32, 0.16] {
        draw_cube(
            center + vec3(offset, -0.08, -0.15) * scale,
            vec3(0.14, 0.12, 0.07) * scale,
            None,
            wheel_color,
        );
    }
}
