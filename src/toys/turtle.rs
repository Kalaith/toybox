use super::library::{brighten, darken, draw_eye_pair, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Low dome shell with a darker rim and plate studs.
    draw_toy_sphere(
        center + vec3(0.0, 0.02, 0.02) * scale,
        0.22 * scale,
        None,
        darken(color, 0.05),
    );
    draw_cube(
        center + vec3(0.0, -0.02, 0.02) * scale,
        vec3(0.44, 0.08, 0.40) * scale,
        None,
        darken(color, 0.16),
    );
    // Scute pattern: one crown plate ringed by five more down the dome.
    draw_cube(
        center + vec3(0.0, 0.19, 0.02) * scale,
        vec3(0.09, 0.045, 0.09) * scale,
        None,
        brighten(color, 0.12),
    );
    for index in 0..5 {
        let angle = index as f32 / 5.0 * std::f32::consts::TAU + 0.63;
        draw_cube(
            center + vec3(angle.cos() * 0.115, 0.135, 0.02 + angle.sin() * 0.115) * scale,
            vec3(0.075, 0.035, 0.075) * scale,
            None,
            brighten(color, if index % 2 == 0 { 0.08 } else { 0.14 }),
        );
    }

    // Cream plastron peeking under the rim, and a tail nub at the back.
    draw_cube(
        center + vec3(0.0, -0.075, 0.02) * scale,
        vec3(0.30, 0.035, 0.28) * scale,
        None,
        Color::new(0.90, 0.82, 0.62, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.0, -0.03, 0.245) * scale,
        0.035 * scale,
        None,
        brighten(color, 0.08),
    );

    // Head poking out the front, feet at the corners.
    let head = center + vec3(0.0, 0.03, -0.26) * scale;
    draw_toy_sphere(head, 0.09 * scale, None, brighten(color, 0.14));
    draw_eye_pair(center, 0.07, -0.33, 0.05, scale);
    // Blush and a little smile.
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.065, 0.02, -0.315) * scale,
            0.020 * scale,
            None,
            Color::new(0.96, 0.64, 0.64, 1.0),
        );
    }
    draw_cube(
        center + vec3(0.0, -0.01, -0.345) * scale,
        vec3(0.040, 0.012, 0.012) * scale,
        None,
        Color::new(0.035, 0.030, 0.026, 1.0),
    );
    for x in [-0.17_f32, 0.17] {
        for z in [-0.14_f32, 0.18] {
            draw_cube(
                center + vec3(x, -0.05, z) * scale,
                vec3(0.09, 0.07, 0.09) * scale,
                None,
                brighten(color, 0.12),
            );
        }
    }
}
