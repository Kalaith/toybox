use super::library::{brighten, darken, draw_cube_with_edges, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Tall rocket-shaped robot with landing fins and a nose dome.
    draw_cube_with_edges(
        center + vec3(0.0, 0.16, 0.0) * scale,
        vec3(0.20, 0.30, 0.20) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.38, 0.0) * scale,
        vec3(0.16, 0.16, 0.16) * scale,
        brighten(color, 0.08),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.52, 0.0) * scale,
        0.09 * scale,
        None,
        Color::new(0.90, 0.86, 0.80, 1.0),
    );

    // Porthole eye and three landing fins.
    draw_toy_sphere(
        center + vec3(0.0, 0.38, -0.09) * scale,
        0.045 * scale,
        None,
        Color::new(0.42, 0.95, 0.96, 1.0),
    );
    for index in 0..3 {
        let angle = index as f32 / 3.0 * std::f32::consts::TAU + 0.52;
        draw_cube_with_edges(
            center + vec3(angle.cos() * 0.15, 0.02, angle.sin() * 0.15) * scale,
            vec3(0.06, 0.18, 0.06) * scale,
            darken(color, 0.14),
        );
    }
    // Thruster bell glow.
    draw_toy_sphere(
        center + vec3(0.0, -0.02, 0.0) * scale,
        0.07 * scale,
        None,
        Color::new(0.95, 0.58, 0.24, 1.0),
    );
}
