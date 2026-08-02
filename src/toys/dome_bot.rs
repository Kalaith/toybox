use super::primitives::{brighten, darken, draw_cube_with_edges, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Retro vacuum-bot: squat drum body under a glass-dome head.
    draw_cube_with_edges(
        center + vec3(0.0, 0.10, 0.0) * scale,
        vec3(0.40, 0.22, 0.36) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.24, 0.0) * scale,
        vec3(0.34, 0.06, 0.30) * scale,
        darken(color, 0.12),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.34, 0.0) * scale,
        0.15 * scale,
        None,
        Color::new(0.72, 0.86, 0.92, 0.96),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.32, -0.06) * scale,
        0.05 * scale,
        None,
        Color::new(0.95, 0.76, 0.30, 1.0),
    );

    // Bumper skirt and little side sensors.
    draw_cube_with_edges(
        center + vec3(0.0, -0.02, 0.0) * scale,
        vec3(0.46, 0.08, 0.42) * scale,
        darken(color, 0.18),
    );
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.24, 0.16, -0.12) * scale,
            0.035 * scale,
            None,
            Color::new(0.92, 0.30, 0.24, 1.0),
        );
    }
    draw_cube_with_edges(
        center + vec3(0.0, 0.12, -0.20) * scale,
        vec3(0.14, 0.05, 0.05) * scale,
        brighten(color, 0.12),
    );

    // Caster wheels peeking out under the bumper skirt.
    let caster = Color::new(0.16, 0.16, 0.18, 1.0);
    for (x, z) in [(0.0_f32, -0.16_f32), (-0.14, 0.12), (0.14, 0.12)] {
        draw_toy_sphere(
            center + vec3(x, -0.07, z) * scale,
            0.035 * scale,
            None,
            caster,
        );
    }

    // Control panel above the bumper: dark screen, teal and gold buttons.
    draw_cube(
        center + vec3(0.0, 0.185, -0.185) * scale,
        vec3(0.10, 0.05, 0.012) * scale,
        None,
        Color::new(0.08, 0.10, 0.12, 1.0),
    );
    draw_toy_sphere(
        center + vec3(-0.085, 0.185, -0.185) * scale,
        0.014 * scale,
        None,
        Color::new(0.56, 0.94, 0.88, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.085, 0.185, -0.185) * scale,
        0.014 * scale,
        None,
        Color::new(0.95, 0.76, 0.30, 1.0),
    );

    // Exhaust vents along both sides.
    for side in [-1.0_f32, 1.0] {
        for z in [0.02_f32, 0.10] {
            draw_cube(
                center + vec3(side * 0.203, 0.10, z) * scale,
                vec3(0.012, 0.055, 0.045) * scale,
                None,
                Color::new(0.10, 0.12, 0.14, 1.0),
            );
        }
    }

    // Beacon on the dome crown.
    draw_cube(
        center + vec3(0.0, 0.495, 0.0) * scale,
        vec3(0.018, 0.030, 0.018) * scale,
        None,
        Color::new(0.80, 0.84, 0.84, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.525, 0.0) * scale,
        0.024 * scale,
        None,
        Color::new(0.92, 0.30, 0.24, 1.0),
    );
}
