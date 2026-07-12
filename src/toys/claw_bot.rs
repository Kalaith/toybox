use super::library::{brighten, darken, draw_cube_with_edges, draw_robot_core, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);
    let metal = Color::new(0.90, 0.92, 0.92, 1.0);

    for side in [-1.0_f32, 1.0] {
        draw_cube_with_edges(
            center + vec3(side * 0.28, 0.12, -0.09) * scale,
            vec3(0.08, 0.26, 0.08) * scale,
            darken(color, 0.12),
        );
        // Elbow joint between arm and gripper.
        draw_toy_sphere(
            center + vec3(side * 0.28, -0.03, -0.10) * scale,
            0.050 * scale,
            None,
            metal,
        );
        // U-shaped gripper: crossbar with two downward prongs.
        draw_cube(
            center + vec3(side * 0.34, -0.05, -0.13) * scale,
            vec3(0.16, 0.05, 0.05) * scale,
            None,
            metal,
        );
        for dx in [0.29_f32, 0.39] {
            draw_cube(
                center + vec3(side * dx, -0.105, -0.14) * scale,
                vec3(0.042, 0.11, 0.042) * scale,
                None,
                metal,
            );
        }
    }

    // Prize block caught in the right gripper.
    draw_cube_with_edges(
        center + vec3(0.34, -0.11, -0.14) * scale,
        vec3(0.068, 0.068, 0.068) * scale,
        brighten(color, 0.18),
    );

    // Wide stance feet keep the heavy arms balanced.
    for x in [-0.10_f32, 0.10] {
        draw_cube_with_edges(
            center + vec3(x, -0.155, 0.01) * scale,
            vec3(0.15, 0.09, 0.26) * scale,
            darken(color, 0.16),
        );
    }
}
