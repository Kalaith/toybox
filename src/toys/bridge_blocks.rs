use super::library::brighten;
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_cube(
        center + vec3(0.0, 0.10, 0.0) * scale,
        vec3(0.58, 0.12, 0.22) * scale,
        None,
        color,
    );
    for offset in [-0.22_f32, 0.0, 0.22] {
        draw_cube(
            center + vec3(offset, -0.10, 0.0) * scale,
            vec3(0.10, 0.28, 0.18) * scale,
            None,
            brighten(color, 0.08),
        );
    }
}
