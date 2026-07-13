use super::library::{darken, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let belly = Color::new(0.95, 0.94, 0.90, 1.0);
    let beak = Color::new(0.95, 0.66, 0.24, 1.0);

    // Tall upright body, white belly front.
    draw_toy_sphere(
        center + vec3(0.0, 0.12, 0.0) * scale,
        0.21 * scale,
        None,
        color,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.34, 0.0) * scale,
        0.14 * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.0, 0.12, -0.155) * scale,
        vec3(0.17, 0.26, 0.07) * scale,
        None,
        belly,
    );

    // Flippers angled out, orange feet and beak.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.21, 0.10, 0.0) * scale,
            vec3(0.05, 0.22, 0.12) * scale,
            None,
            darken(color, 0.12),
        );
        draw_cube(
            center + vec3(side * 0.09, -0.10, -0.04) * scale,
            vec3(0.09, 0.05, 0.13) * scale,
            None,
            beak,
        );
        // White face patch behind each eye.
        draw_toy_sphere(
            center + vec3(side * 0.055, 0.375, -0.102) * scale,
            0.050 * scale,
            None,
            belly,
        );
        draw_toy_sphere(
            center + vec3(side * 0.06, 0.38, -0.115) * scale,
            0.025 * scale,
            None,
            Color::new(0.07, 0.06, 0.05, 1.0),
        );
    }
    draw_cube(
        center + vec3(0.0, 0.32, -0.15) * scale,
        vec3(0.05, 0.05, 0.08) * scale,
        None,
        beak,
    );

    // Knit beanie with a cream pompom, plus a matching scarf.
    let knit = Color::new(0.80, 0.24, 0.24, 1.0);
    draw_cube(
        center + vec3(0.0, 0.455, 0.0) * scale,
        vec3(0.15, 0.04, 0.15) * scale,
        None,
        knit,
    );
    draw_cube(
        center + vec3(0.0, 0.50, 0.0) * scale,
        vec3(0.11, 0.055, 0.11) * scale,
        None,
        knit,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.545, 0.0) * scale,
        0.035 * scale,
        None,
        Color::new(0.94, 0.90, 0.80, 1.0),
    );
    draw_cube(
        center + vec3(0.0, 0.25, 0.0) * scale,
        vec3(0.19, 0.05, 0.19) * scale,
        None,
        knit,
    );
    draw_cube(
        center + vec3(0.065, 0.18, -0.165) * scale,
        vec3(0.05, 0.09, 0.022) * scale,
        None,
        knit,
    );
}
