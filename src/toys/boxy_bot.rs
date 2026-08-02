use super::primitives::{brighten, darken, draw_cube_with_edges, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // One oversized chunky cube of a robot with stubby limbs.
    draw_cube_with_edges(
        center + vec3(0.0, 0.22, 0.0) * scale,
        vec3(0.40, 0.40, 0.34) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.26, -0.175) * scale,
        vec3(0.28, 0.18, 0.04) * scale,
        darken(color, 0.20),
    );
    for side in [-1.0_f32, 1.0] {
        // Square eyes on the face panel.
        draw_cube_with_edges(
            center + vec3(side * 0.08, 0.30, -0.20) * scale,
            vec3(0.06, 0.06, 0.02) * scale,
            Color::new(0.95, 0.86, 0.40, 1.0),
        );
        // Blocky arms and feet.
        draw_cube_with_edges(
            center + vec3(side * 0.26, 0.18, 0.0) * scale,
            vec3(0.10, 0.24, 0.12) * scale,
            darken(color, 0.10),
        );
        draw_toy_sphere(
            center + vec3(side * 0.26, 0.03, 0.0) * scale,
            0.055 * scale,
            None,
            Color::new(0.80, 0.84, 0.84, 1.0),
        );
        draw_cube_with_edges(
            center + vec3(side * 0.11, -0.04, 0.0) * scale,
            vec3(0.13, 0.12, 0.18) * scale,
            darken(color, 0.14),
        );
    }
    // Speaker-grill mouth under the eyes.
    for x in [-0.05_f32, 0.0, 0.05] {
        draw_cube(
            center + vec3(x, 0.205, -0.20) * scale,
            vec3(0.025, 0.05, 0.02) * scale,
            None,
            Color::new(0.75, 0.66, 0.32, 1.0),
        );
    }
    // Belly access hatch with a silver latch.
    draw_cube_with_edges(
        center + vec3(0.0, 0.075, -0.175) * scale,
        vec3(0.16, 0.10, 0.02) * scale,
        darken(color, 0.06),
    );
    draw_toy_sphere(
        center + vec3(0.055, 0.075, -0.19) * scale,
        0.018 * scale,
        None,
        Color::new(0.82, 0.84, 0.80, 1.0),
    );
    // Rivet strip across the top.
    draw_cube_with_edges(
        center + vec3(0.0, 0.44, 0.0) * scale,
        vec3(0.20, 0.05, 0.14) * scale,
        brighten(color, 0.10),
    );
    // Gold wind-up key on the back.
    let key_gold = Color::new(0.90, 0.72, 0.30, 1.0);
    draw_cube_with_edges(
        center + vec3(0.0, 0.26, 0.215) * scale,
        vec3(0.045, 0.045, 0.10) * scale,
        key_gold,
    );
    for x in [-0.055_f32, 0.055] {
        draw_cube_with_edges(
            center + vec3(x, 0.26, 0.28) * scale,
            vec3(0.03, 0.16, 0.03) * scale,
            key_gold,
        );
    }
    for y in [0.175_f32, 0.345] {
        draw_cube_with_edges(
            center + vec3(0.0, y, 0.28) * scale,
            vec3(0.14, 0.03, 0.03) * scale,
            key_gold,
        );
    }
}
