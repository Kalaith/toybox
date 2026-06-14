use super::library::{brighten, darken, draw_cube_with_edges, draw_wheel};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_cube_with_edges(
        center + vec3(-0.10, 0.04, 0.0) * scale,
        vec3(0.42, 0.22, 0.22) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.18, 0.14, 0.0) * scale,
        vec3(0.18, 0.20, 0.20) * scale,
        brighten(color, 0.08),
    );
    let wheel_color = darken(color, 0.22);
    draw_cube_with_edges(
        center + vec3(-0.06, -0.10, -0.10) * scale,
        vec3(0.54, 0.08, 0.08) * scale,
        wheel_color,
    );
    for offset in [-0.22_f32, 0.16] {
        draw_wheel(
            center + vec3(offset, -0.08, -0.15) * scale,
            0.070 * scale,
            0.030 * scale,
            wheel_color,
        );
    }
    draw_cube(
        center + vec3(0.27, 0.28, 0.0) * scale,
        vec3(0.035, 0.13, 0.06) * scale,
        None,
        darken(color, 0.12),
    );
}
