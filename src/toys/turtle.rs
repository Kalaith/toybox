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
    for (x, z) in [(0.0, 0.02), (-0.10, 0.10), (0.10, 0.10), (0.0, -0.08)] {
        draw_cube(
            center + vec3(x, 0.16, z) * scale,
            vec3(0.08, 0.04, 0.08) * scale,
            None,
            brighten(color, 0.10),
        );
    }

    // Head poking out the front, feet at the corners.
    let head = center + vec3(0.0, 0.03, -0.26) * scale;
    draw_toy_sphere(head, 0.09 * scale, None, brighten(color, 0.14));
    draw_eye_pair(center, 0.07, -0.33, 0.05, scale);
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
