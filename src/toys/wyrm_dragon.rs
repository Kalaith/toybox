use super::library::{brighten, darken, draw_eye_pair, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Wingless serpent: body segments winding in an S-curve, head raised.
    let segments = [
        (vec3(0.0, 0.26, -0.26), 0.13),
        (vec3(0.10, 0.14, -0.12), 0.14),
        (vec3(0.0, 0.10, 0.04), 0.145),
        (vec3(-0.12, 0.08, 0.20), 0.13),
        (vec3(-0.02, 0.06, 0.35), 0.10),
        (vec3(0.10, 0.05, 0.44), 0.07),
    ];
    for (index, (offset, radius)) in segments.iter().enumerate() {
        let tone = if index % 2 == 0 {
            color
        } else {
            brighten(color, 0.08)
        };
        draw_toy_sphere(center + *offset * scale, radius * scale, None, tone);
    }

    // Back ridge fins along the spine.
    for (offset, _) in segments.iter().take(4) {
        draw_cube(
            center + (*offset + vec3(0.0, 0.13, 0.0)) * scale,
            vec3(0.035, 0.09, 0.06) * scale,
            None,
            darken(color, 0.14),
        );
    }

    draw_eye_pair(center, 0.30, -0.37, 0.06, scale);
}
