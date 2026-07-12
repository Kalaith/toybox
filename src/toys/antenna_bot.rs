use super::library::draw_toy_sphere;
use super::library::{brighten, darken, draw_cube_with_edges, draw_robot_arms, draw_robot_core};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);
    draw_robot_arms(center, color, scale);

    // Twin staggered antennae: a tall gold broadcast mast and a short
    // teal receiver, so the bot reads as a radio even in silhouette.
    let mast_color = Color::new(0.10, 0.12, 0.14, 1.0);
    draw_cube_with_edges(
        center + vec3(-0.09, 0.62, -0.01) * scale,
        vec3(0.035, 0.30, 0.035) * scale,
        mast_color,
    );
    draw_toy_sphere(
        center + vec3(-0.09, 0.80, -0.01) * scale,
        0.065 * scale,
        None,
        Color::new(0.95, 0.76, 0.30, 1.0),
    );
    draw_cube_with_edges(
        center + vec3(0.09, 0.57, -0.01) * scale,
        vec3(0.035, 0.20, 0.035) * scale,
        mast_color,
    );
    draw_toy_sphere(
        center + vec3(0.09, 0.70, -0.01) * scale,
        0.048 * scale,
        None,
        Color::new(0.56, 0.94, 0.88, 1.0),
    );

    // Fading signal pips rising off the broadcast mast.
    draw_toy_sphere(
        center + vec3(-0.09, 0.90, -0.01) * scale,
        0.030 * scale,
        None,
        Color::new(0.97, 0.82, 0.42, 0.55),
    );
    draw_toy_sphere(
        center + vec3(-0.09, 0.98, -0.01) * scale,
        0.040 * scale,
        None,
        Color::new(0.98, 0.86, 0.52, 0.28),
    );

    // Shoulder caps over the arm joints.
    for x in [-0.26_f32, 0.26] {
        draw_cube_with_edges(
            center + vec3(x, 0.25, 0.0) * scale,
            vec3(0.13, 0.055, 0.14) * scale,
            brighten(color, 0.06),
        );
    }

    // Tuning dial beside the chest screen.
    draw_toy_sphere(
        center + vec3(0.11, -0.01, -0.155) * scale,
        0.032 * scale,
        None,
        Color::new(0.95, 0.76, 0.30, 1.0),
    );

    // Two stubby boots with silver toe caps instead of the old base slab.
    for x in [-0.09_f32, 0.09] {
        draw_cube_with_edges(
            center + vec3(x, -0.15, 0.02) * scale,
            vec3(0.13, 0.10, 0.28) * scale,
            darken(color, 0.16),
        );
        draw_toy_sphere(
            center + vec3(x, -0.16, -0.13) * scale,
            0.045 * scale,
            None,
            Color::new(0.80, 0.84, 0.84, 1.0),
        );
    }
}
