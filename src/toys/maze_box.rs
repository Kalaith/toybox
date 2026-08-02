use super::primitives::draw_toy_sphere;
use super::primitives::{darken, draw_game_box};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);

    // Labyrinth tray molded onto the lid: outer rim with an entrance
    // gap at the south-east, then staggered inner walls.
    let wall = darken(color, 0.24);
    let wall_y = 0.115;
    let walls = [
        // Outer rim (south wall stops short: that's the entrance).
        (0.0_f32, -0.15_f32, 0.44_f32, 0.03_f32),
        (-0.06, 0.15, 0.32, 0.03),
        (-0.22, 0.0, 0.03, 0.33),
        (0.22, 0.0, 0.03, 0.33),
        // Inner baffles.
        (-0.08, -0.02, 0.03, 0.20),
        (0.04, 0.06, 0.18, 0.03),
        (0.10, -0.09, 0.03, 0.14),
        (-0.14, -0.09, 0.13, 0.03),
    ];
    for (x, z, w, d) in walls {
        draw_cube(
            center + vec3(x, wall_y, z) * scale,
            vec3(w, 0.05, d) * scale,
            None,
            wall,
        );
    }

    // Goal hole at the heart of the maze.
    draw_cube(
        center + vec3(-0.02, 0.104, -0.01) * scale,
        vec3(0.055, 0.010, 0.055) * scale,
        None,
        Color::new(0.08, 0.09, 0.10, 1.0),
    );

    // Gold marble mid-run, silver one waiting at the entrance.
    draw_toy_sphere(
        center + vec3(0.16, 0.13, 0.09) * scale,
        0.035 * scale,
        None,
        Color::new(0.94, 0.72, 0.24, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.185, 0.125, 0.175) * scale,
        0.027 * scale,
        None,
        Color::new(0.82, 0.84, 0.86, 1.0),
    );
}
