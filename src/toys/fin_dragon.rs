use super::primitives::{draw_cube_with_edges, draw_dragon_base};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);
    let fin = Color::new(0.96, 0.88, 0.58, 1.0);
    for index in 0..4 {
        draw_cube_with_edges(
            center + vec3(0.0, 0.29 - index as f32 * 0.04, -0.12 + index as f32 * 0.13) * scale,
            vec3(0.08, 0.14, 0.05) * scale,
            fin,
        );
    }
    // Caudal fan: three blades spreading from the tail end.
    draw_cube(
        center + vec3(0.0, 0.12, 0.24) * scale,
        vec3(0.11, 0.10, 0.045) * scale,
        None,
        fin,
    );
    for y in [0.205_f32, 0.04] {
        draw_cube(
            center + vec3(0.0, y, 0.27) * scale,
            vec3(0.085, 0.065, 0.04) * scale,
            None,
            fin,
        );
    }

    // Swept cheek fins off the head, gill-style.
    let fin_edge = Color::new(0.98, 0.94, 0.72, 1.0);
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.17, 0.19, -0.30) * scale,
            vec3(0.09, 0.065, 0.028) * scale,
            None,
            fin,
        );
        draw_cube(
            center + vec3(side * 0.215, 0.19, -0.30) * scale,
            vec3(0.055, 0.040, 0.020) * scale,
            None,
            fin_edge,
        );
    }
}
