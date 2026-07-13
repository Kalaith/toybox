use super::library::{darken, draw_cube_with_edges, draw_dragon_base, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);
    let horn = Color::new(0.96, 0.88, 0.58, 1.0);
    draw_cube_with_edges(
        center + vec3(-0.16, 0.28, -0.35) * scale,
        vec3(0.07, 0.24, 0.06) * scale,
        horn,
    );
    draw_cube_with_edges(
        center + vec3(0.16, 0.28, -0.35) * scale,
        vec3(0.07, 0.24, 0.06) * scale,
        horn,
    );
    draw_cube(
        center + vec3(0.0, 0.13, -0.45) * scale,
        vec3(0.12, 0.06, 0.14) * scale,
        None,
        darken(color, 0.06),
    );
    for x in [-0.04_f32, 0.04] {
        draw_cube(
            center + vec3(x, 0.15, -0.53) * scale,
            vec3(0.026, 0.022, 0.020) * scale,
            None,
            Color::new(0.04, 0.03, 0.025, 1.0),
        );
    }

    // Tapered tips angling outward off the main horns.
    for side in [-1.0_f32, 1.0] {
        draw_cube_with_edges(
            center + vec3(side * 0.185, 0.455, -0.35) * scale,
            vec3(0.045, 0.14, 0.04) * scale,
            horn,
        );
    }

    // Brow horns over the eyes and a rhino horn on the snout.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.08, 0.33, -0.36) * scale,
            vec3(0.035, 0.08, 0.035) * scale,
            None,
            horn,
        );
    }
    draw_cube(
        center + vec3(0.0, 0.20, -0.46) * scale,
        vec3(0.035, 0.09, 0.035) * scale,
        None,
        horn,
    );

    // Cheek studs and shoulder spikes round out the armored look.
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.13, 0.16, -0.36) * scale,
            0.028 * scale,
            None,
            horn,
        );
        draw_cube(
            center + vec3(side * 0.14, 0.26, -0.05) * scale,
            vec3(0.04, 0.09, 0.04) * scale,
            None,
            horn,
        );
    }
}
