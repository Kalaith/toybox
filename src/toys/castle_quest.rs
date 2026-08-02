use super::primitives::{draw_game_box, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);
    let stone = Color::new(0.82, 0.82, 0.78, 1.0);
    draw_cube(
        center + vec3(-0.15, 0.17, -0.02) * scale,
        vec3(0.09, 0.13, 0.09) * scale,
        None,
        stone,
    );
    draw_cube(
        center + vec3(0.15, 0.17, -0.02) * scale,
        vec3(0.09, 0.13, 0.09) * scale,
        None,
        stone,
    );
    draw_cube(
        center + vec3(0.0, 0.15, -0.02) * scale,
        vec3(0.13, 0.09, 0.09) * scale,
        None,
        stone,
    );
    draw_cube(
        center + vec3(0.0, 0.24, -0.02) * scale,
        vec3(0.07, 0.07, 0.07) * scale,
        None,
        Color::new(0.72, 0.22, 0.20, 1.0),
    );
    for x in [-0.15_f32, 0.15] {
        draw_cube(
            center + vec3(x, 0.25, -0.02) * scale,
            vec3(0.055, 0.050, 0.070) * scale,
            None,
            Color::new(0.72, 0.22, 0.20, 1.0),
        );
        // Tower windows.
        draw_cube(
            center + vec3(x, 0.16, -0.068) * scale,
            vec3(0.022, 0.035, 0.010) * scale,
            None,
            Color::new(0.16, 0.14, 0.14, 1.0),
        );
    }

    // Gold pennant over the keep.
    draw_cube(
        center + vec3(0.0, 0.305, -0.02) * scale,
        vec3(0.010, 0.065, 0.010) * scale,
        None,
        Color::new(0.30, 0.24, 0.20, 1.0),
    );
    draw_cube(
        center + vec3(0.026, 0.320, -0.02) * scale,
        vec3(0.042, 0.022, 0.008) * scale,
        None,
        Color::new(0.95, 0.78, 0.30, 1.0),
    );

    // Player meeples and a die scattered on the lid.
    let meeples = [
        (-0.19_f32, Color::new(0.26, 0.46, 0.86, 1.0)),
        (0.19, Color::new(0.95, 0.78, 0.30, 1.0)),
    ];
    for (x, meeple_color) in meeples {
        draw_cube(
            center + vec3(x, 0.125, 0.11) * scale,
            vec3(0.038, 0.052, 0.028) * scale,
            None,
            meeple_color,
        );
        draw_toy_sphere(
            center + vec3(x, 0.163, 0.11) * scale,
            0.023 * scale,
            None,
            meeple_color,
        );
    }
    draw_cube(
        center + vec3(0.09, 0.115, 0.13) * scale,
        vec3(0.036, 0.036, 0.036) * scale,
        None,
        Color::new(0.94, 0.93, 0.90, 1.0),
    );
    draw_cube(
        center + vec3(0.09, 0.134, 0.13) * scale,
        vec3(0.011, 0.006, 0.011) * scale,
        None,
        Color::new(0.14, 0.14, 0.16, 1.0),
    );
}
