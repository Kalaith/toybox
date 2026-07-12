use super::library::{brighten, draw_eye_pair, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let shell = Color::new(0.93, 0.90, 0.82, 1.0);

    // Cracked egg shell base with a zigzag rim of shard cubes.
    draw_toy_sphere(
        center + vec3(0.0, 0.02, 0.0) * scale,
        0.20 * scale,
        None,
        shell,
    );
    for index in 0..7 {
        let angle = index as f32 / 7.0 * std::f32::consts::TAU;
        let lift = if index % 2 == 0 { 0.17 } else { 0.11 };
        draw_cube(
            center + vec3(angle.cos() * 0.15, lift, angle.sin() * 0.15) * scale,
            vec3(0.07, 0.09, 0.07) * scale,
            None,
            shell,
        );
    }

    // Hatchling head and tiny arms poking out the top.
    draw_toy_sphere(
        center + vec3(0.0, 0.26, -0.02) * scale,
        0.13 * scale,
        None,
        color,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.22, -0.13) * scale,
        0.06 * scale,
        None,
        brighten(color, 0.12),
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.14, 0.20, 0.0) * scale,
            vec3(0.05, 0.08, 0.05) * scale,
            None,
            brighten(color, 0.05),
        );
        draw_cube(
            center + vec3(side * 0.06, 0.40, 0.0) * scale,
            vec3(0.03, 0.06, 0.03) * scale,
            None,
            Color::new(0.96, 0.88, 0.58, 1.0),
        );
    }
    draw_eye_pair(center, 0.29, -0.13, 0.055, scale);
}
