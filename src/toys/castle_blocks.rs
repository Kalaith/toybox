use super::library::{brighten, darken, draw_cube_with_edges, draw_studded_block};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_studded_block(
        center + vec3(0.0, -0.08, 0.0) * scale,
        vec3(0.44, 0.18, 0.24) * scale,
        color,
    );
    draw_studded_block(
        center + vec3(-0.18, 0.12, 0.0) * scale,
        vec3(0.14, 0.28, 0.20) * scale,
        brighten(color, 0.08),
    );
    draw_studded_block(
        center + vec3(0.18, 0.12, 0.0) * scale,
        vec3(0.14, 0.28, 0.20) * scale,
        brighten(color, 0.08),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.18, 0.0) * scale,
        vec3(0.16, 0.16, 0.20) * scale,
        darken(color, 0.10),
    );

    // Gate and doorstep on the front of the base.
    draw_cube(
        center + vec3(0.0, -0.10, -0.125) * scale,
        vec3(0.10, 0.13, 0.02) * scale,
        None,
        Color::new(0.18, 0.13, 0.10, 1.0),
    );
    draw_cube_with_edges(
        center + vec3(0.0, -0.16, -0.16) * scale,
        vec3(0.14, 0.05, 0.08) * scale,
        darken(color, 0.14),
    );

    // Arrow-slit windows on the towers.
    for x in [-0.18_f32, 0.18] {
        draw_cube(
            center + vec3(x, 0.14, -0.105) * scale,
            vec3(0.025, 0.07, 0.015) * scale,
            None,
            Color::new(0.14, 0.12, 0.12, 1.0),
        );
    }

    // Merlons around the keep roof.
    for x in [-0.055_f32, 0.055] {
        for z in [-0.075_f32, 0.075] {
            draw_cube_with_edges(
                center + vec3(x, 0.285, z) * scale,
                vec3(0.035, 0.05, 0.035) * scale,
                darken(color, 0.06),
            );
        }
    }

    // Banner flying from the keep.
    draw_cube_with_edges(
        center + vec3(0.0, 0.36, 0.0) * scale,
        vec3(0.016, 0.13, 0.016) * scale,
        Color::new(0.42, 0.30, 0.22, 1.0),
    );
    draw_cube(
        center + vec3(0.045, 0.40, 0.0) * scale,
        vec3(0.075, 0.04, 0.012) * scale,
        None,
        Color::new(0.82, 0.24, 0.20, 1.0),
    );
}
