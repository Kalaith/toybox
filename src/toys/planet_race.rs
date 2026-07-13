use super::library::draw_game_box;
use super::library::draw_toy_sphere;
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);
    draw_toy_sphere(
        center + vec3(-0.12, 0.15, -0.04) * scale,
        0.07 * scale,
        None,
        Color::new(0.40, 0.70, 0.96, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.08, 0.15, -0.03) * scale,
        0.05 * scale,
        None,
        Color::new(0.96, 0.66, 0.30, 1.0),
    );
    // Saturn-style rings around the orange planet.
    let ring = Color::new(0.90, 0.80, 0.58, 1.0);
    draw_cube(
        center + vec3(0.08, 0.15, -0.03) * scale,
        vec3(0.15, 0.008, 0.09) * scale,
        None,
        ring,
    );
    draw_cube(
        center + vec3(0.08, 0.15, -0.03) * scale,
        vec3(0.09, 0.008, 0.15) * scale,
        None,
        ring,
    );

    // Racing rocket: white body, red nose and tail fins, exhaust flame.
    let hull = Color::new(0.94, 0.94, 0.88, 1.0);
    let trim = Color::new(0.82, 0.24, 0.20, 1.0);
    draw_cube(
        center + vec3(0.18, 0.13, 0.05) * scale,
        vec3(0.10, 0.034, 0.034) * scale,
        None,
        hull,
    );
    draw_cube(
        center + vec3(0.245, 0.13, 0.05) * scale,
        vec3(0.028, 0.028, 0.028) * scale,
        None,
        trim,
    );
    draw_cube(
        center + vec3(0.135, 0.13, 0.05) * scale,
        vec3(0.018, 0.058, 0.018) * scale,
        None,
        trim,
    );
    draw_cube(
        center + vec3(0.135, 0.13, 0.05) * scale,
        vec3(0.018, 0.018, 0.058) * scale,
        None,
        trim,
    );
    draw_cube(
        center + vec3(0.112, 0.13, 0.05) * scale,
        vec3(0.022, 0.022, 0.022) * scale,
        None,
        Color::new(0.96, 0.66, 0.30, 1.0),
    );
    draw_cube(
        center + vec3(-0.01, 0.145, -0.03) * scale,
        vec3(0.27, 0.012, 0.020) * scale,
        None,
        Color::new(0.88, 0.92, 1.0, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.20, 0.155, -0.07) * scale,
        0.024 * scale,
        None,
        Color::new(0.96, 0.90, 0.46, 1.0),
    );

    // A grey moon by the blue planet and a sprinkle of stars.
    draw_toy_sphere(
        center + vec3(-0.19, 0.17, -0.10) * scale,
        0.022 * scale,
        None,
        Color::new(0.78, 0.78, 0.80, 1.0),
    );
    for (x, z) in [(-0.03_f32, 0.10_f32), (0.02, -0.12)] {
        draw_toy_sphere(
            center + vec3(x, 0.16, z) * scale,
            0.014 * scale,
            None,
            Color::new(0.96, 0.90, 0.46, 1.0),
        );
    }
}
