use super::library::{draw_studded_block, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Five blocks stacked into an arch, wearing actual rainbow colors
    // nudged a little toward the toy's palette tone so siblings vary.
    let tint = |r: f32, g: f32, b: f32| {
        Color::new(
            r * 0.86 + color.r * 0.14,
            g * 0.86 + color.g * 0.14,
            b * 0.86 + color.b * 0.14,
            1.0,
        )
    };
    let arc = [
        (
            vec3(-0.22, 0.02, 0.0),
            vec3(0.14, 0.20, 0.20),
            tint(0.88, 0.30, 0.26),
        ),
        (
            vec3(0.22, 0.02, 0.0),
            vec3(0.14, 0.20, 0.20),
            tint(0.36, 0.58, 0.90),
        ),
        (
            vec3(-0.19, 0.20, 0.0),
            vec3(0.13, 0.16, 0.18),
            tint(0.95, 0.60, 0.22),
        ),
        (
            vec3(0.19, 0.20, 0.0),
            vec3(0.13, 0.16, 0.18),
            tint(0.44, 0.76, 0.38),
        ),
        (
            vec3(0.0, 0.33, 0.0),
            vec3(0.42, 0.13, 0.16),
            tint(0.96, 0.82, 0.30),
        ),
    ];
    for (offset, size, block_color) in arc {
        draw_studded_block(center + offset * scale, size * scale, block_color);
    }

    // Puffy clouds at both feet of the rainbow.
    let cloud = Color::new(0.95, 0.95, 0.97, 1.0);
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.24, -0.10, 0.0) * scale,
            0.068 * scale,
            None,
            cloud,
        );
        draw_toy_sphere(
            center + vec3(side * 0.31, -0.115, 0.03) * scale,
            0.052 * scale,
            None,
            cloud,
        );
        draw_toy_sphere(
            center + vec3(side * 0.17, -0.12, -0.04) * scale,
            0.048 * scale,
            None,
            cloud,
        );
    }
}
