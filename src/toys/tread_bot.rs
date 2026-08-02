use super::primitives::{darken, draw_cube_with_edges, draw_robot_core, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);

    // Two side track pods with plates and drive wheels, bridged by a
    // belly plate, instead of the old single slab.
    let track = Color::new(0.08, 0.09, 0.10, 1.0);
    let plate = Color::new(0.78, 0.82, 0.84, 1.0);
    draw_cube_with_edges(
        center + vec3(0.0, -0.13, 0.0) * scale,
        vec3(0.18, 0.08, 0.30) * scale,
        darken(color, 0.10),
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube_with_edges(
            center + vec3(side * 0.17, -0.15, 0.0) * scale,
            vec3(0.15, 0.12, 0.38) * scale,
            track,
        );
        for z in [-0.13_f32, 0.0, 0.13] {
            draw_cube(
                center + vec3(side * 0.17, -0.085, z) * scale,
                vec3(0.10, 0.030, 0.08) * scale,
                None,
                plate,
            );
        }
        for z in [-0.17_f32, 0.17] {
            draw_toy_sphere(
                center + vec3(side * 0.17, -0.16, z) * scale,
                0.045 * scale,
                None,
                plate,
            );
        }
    }

    // Dozer blade out front with hazard stripes.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.14, -0.10, -0.22) * scale,
            vec3(0.030, 0.030, 0.10) * scale,
            None,
            darken(color, 0.14),
        );
    }
    draw_cube_with_edges(
        center + vec3(0.0, -0.10, -0.28) * scale,
        vec3(0.42, 0.13, 0.030) * scale,
        Color::new(0.62, 0.66, 0.68, 1.0),
    );
    for x in [-0.15_f32, 0.05] {
        draw_cube(
            center + vec3(x, -0.10, -0.298) * scale,
            vec3(0.055, 0.11, 0.012) * scale,
            None,
            Color::new(0.93, 0.76, 0.32, 1.0),
        );
    }

    // Exhaust stack behind the shoulder.
    draw_cube(
        center + vec3(0.10, 0.16, 0.18) * scale,
        vec3(0.028, 0.12, 0.028) * scale,
        None,
        Color::new(0.42, 0.45, 0.48, 1.0),
    );
    draw_cube(
        center + vec3(0.10, 0.225, 0.18) * scale,
        vec3(0.036, 0.020, 0.036) * scale,
        None,
        track,
    );
}
