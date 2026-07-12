use super::library::{brighten, darken, draw_eye_pair, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Asleep and curled flat: body spirals inward, head resting on the coil.
    for index in 0..9 {
        let t = index as f32 / 8.0;
        let angle = t * std::f32::consts::TAU * 1.15 + 1.9;
        let radius = 0.26 - t * 0.17;
        let segment_radius = 0.105 - t * 0.045;
        let tone = if index % 2 == 0 {
            color
        } else {
            brighten(color, 0.07)
        };
        draw_toy_sphere(
            center + vec3(angle.cos() * radius, 0.07, angle.sin() * radius) * scale,
            segment_radius * scale,
            None,
            tone,
        );
    }

    // Head resting on top of the outer coil, eyes closed low.
    let head = center + vec3(-0.06, 0.17, -0.22) * scale;
    draw_toy_sphere(head, 0.11 * scale, None, brighten(color, 0.10));
    draw_toy_sphere(
        head + vec3(0.0, -0.01, -0.10) * scale,
        0.05 * scale,
        None,
        brighten(color, 0.16),
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            head + vec3(side * 0.06, 0.10, 0.02) * scale,
            vec3(0.03, 0.06, 0.03) * scale,
            None,
            darken(color, 0.12),
        );
    }
    draw_eye_pair(center, 0.19, -0.31, 0.05, scale);
}
