use super::library::{brighten, darken, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Upright egg body with a pale belly panel.
    draw_toy_sphere(
        center + vec3(0.0, 0.10, 0.0) * scale,
        0.22 * scale,
        None,
        color,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.30, 0.0) * scale,
        0.17 * scale,
        None,
        brighten(color, 0.06),
    );
    draw_cube(
        center + vec3(0.0, 0.10, -0.17) * scale,
        vec3(0.19, 0.22, 0.06) * scale,
        None,
        brighten(color, 0.20),
    );

    // Folded wings and ear tufts.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.22, 0.10, 0.02) * scale,
            vec3(0.06, 0.26, 0.16) * scale,
            None,
            darken(color, 0.10),
        );
        draw_cube(
            center + vec3(side * 0.10, 0.47, 0.0) * scale,
            vec3(0.05, 0.10, 0.05) * scale,
            None,
            darken(color, 0.12),
        );
    }

    // Big round eyes set in darker facial discs, beak between them.
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.08, 0.33, -0.125) * scale,
            0.068 * scale,
            None,
            darken(color, 0.06),
        );
        draw_toy_sphere(
            center + vec3(side * 0.08, 0.33, -0.14) * scale,
            0.055 * scale,
            None,
            Color::new(0.96, 0.94, 0.88, 1.0),
        );
        draw_toy_sphere(
            center + vec3(side * 0.08, 0.33, -0.185) * scale,
            0.028 * scale,
            None,
            Color::new(0.08, 0.07, 0.06, 1.0),
        );
    }
    draw_cube(
        center + vec3(0.0, 0.26, -0.17) * scale,
        vec3(0.05, 0.06, 0.06) * scale,
        None,
        Color::new(0.94, 0.72, 0.30, 1.0),
    );

    // Feather chevrons down the belly panel.
    let chevron = darken(color, 0.05);
    let marks = [
        (-0.045_f32, 0.16_f32),
        (0.045, 0.16),
        (0.0, 0.10),
        (-0.045, 0.04),
        (0.045, 0.04),
    ];
    for (x, y) in marks {
        draw_cube(
            center + vec3(x, y, -0.202) * scale,
            vec3(0.032, 0.016, 0.012) * scale,
            None,
            chevron,
        );
    }

    // Wing-tip bands and little orange feet.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.222, -0.02, 0.02) * scale,
            vec3(0.058, 0.05, 0.15) * scale,
            None,
            brighten(color, 0.08),
        );
        draw_cube(
            center + vec3(side * 0.08, -0.115, -0.10) * scale,
            vec3(0.06, 0.045, 0.09) * scale,
            None,
            Color::new(0.94, 0.72, 0.30, 1.0),
        );
    }
}
