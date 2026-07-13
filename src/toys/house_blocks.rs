use super::library::{darken, draw_studded_block, draw_toy_sphere, shift_block_color};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Little block cottage: walls, stepped roof ridge, chimney, door tile.
    draw_studded_block(
        center + vec3(0.0, 0.06, 0.0) * scale,
        vec3(0.36, 0.26, 0.30) * scale,
        color,
    );
    draw_studded_block(
        center + vec3(0.0, 0.25, 0.0) * scale,
        vec3(0.40, 0.10, 0.34) * scale,
        shift_block_color(color, 1),
    );
    draw_studded_block(
        center + vec3(0.0, 0.34, 0.0) * scale,
        vec3(0.24, 0.09, 0.24) * scale,
        shift_block_color(color, 2),
    );
    draw_studded_block(
        center + vec3(0.0, 0.42, 0.0) * scale,
        vec3(0.10, 0.08, 0.14) * scale,
        shift_block_color(color, 3),
    );

    // Chimney and a dark doorway tile.
    draw_studded_block(
        center + vec3(0.13, 0.42, 0.08) * scale,
        vec3(0.06, 0.14, 0.06) * scale,
        shift_block_color(color, 4),
    );
    draw_cube(
        center + vec3(0.0, 0.02, -0.155) * scale,
        vec3(0.09, 0.16, 0.02) * scale,
        None,
        darken(color, 0.30),
    );
    // Gold doorknob and a doorstep.
    draw_toy_sphere(
        center + vec3(0.03, 0.01, -0.168) * scale,
        0.015 * scale,
        None,
        Color::new(0.93, 0.76, 0.32, 1.0),
    );
    draw_cube(
        center + vec3(0.0, -0.085, -0.175) * scale,
        vec3(0.11, 0.03, 0.05) * scale,
        None,
        darken(color, 0.16),
    );

    // Warm-lit windows with cross mullions flanking the door.
    let mullion = Color::new(0.25, 0.20, 0.16, 1.0);
    for x in [-0.115_f32, 0.115] {
        draw_cube(
            center + vec3(x, 0.09, -0.155) * scale,
            vec3(0.07, 0.07, 0.015) * scale,
            None,
            Color::new(0.97, 0.88, 0.60, 1.0),
        );
        draw_cube(
            center + vec3(x, 0.09, -0.160) * scale,
            vec3(0.07, 0.012, 0.014) * scale,
            None,
            mullion,
        );
        draw_cube(
            center + vec3(x, 0.09, -0.160) * scale,
            vec3(0.012, 0.07, 0.014) * scale,
            None,
            mullion,
        );
    }

    // Smoke puffs drifting off the chimney.
    draw_toy_sphere(
        center + vec3(0.13, 0.55, 0.08) * scale,
        0.035 * scale,
        None,
        Color::new(0.90, 0.90, 0.92, 0.55),
    );
    draw_toy_sphere(
        center + vec3(0.165, 0.63, 0.06) * scale,
        0.046 * scale,
        None,
        Color::new(0.90, 0.90, 0.92, 0.32),
    );

    // Bushes at the front corners.
    for x in [-0.21_f32, 0.21] {
        draw_toy_sphere(
            center + vec3(x, -0.05, -0.13) * scale,
            0.05 * scale,
            None,
            Color::new(0.34, 0.58, 0.32, 1.0),
        );
    }
}
