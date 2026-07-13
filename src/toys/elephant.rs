use super::library::{brighten, darken, draw_eye_pair, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Round body on stubby legs.
    draw_toy_sphere(
        center + vec3(0.0, 0.04, 0.05) * scale,
        0.23 * scale,
        None,
        color,
    );
    for x in [-0.12_f32, 0.12] {
        for z in [-0.08_f32, 0.16] {
            draw_cube(
                center + vec3(x, -0.14, z) * scale,
                vec3(0.10, 0.14, 0.10) * scale,
                None,
                darken(color, 0.08),
            );
        }
    }

    // Head with wide flat ears and a segmented trunk.
    let head = center + vec3(0.0, 0.22, -0.16) * scale;
    draw_toy_sphere(head, 0.15 * scale, None, brighten(color, 0.05));
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            head + vec3(side * 0.18, 0.02, 0.03) * scale,
            vec3(0.05, 0.17, 0.15) * scale,
            None,
            darken(color, 0.10),
        );
    }
    draw_cube(
        head + vec3(0.0, -0.09, -0.13) * scale,
        vec3(0.07, 0.10, 0.07) * scale,
        None,
        color,
    );
    draw_cube(
        head + vec3(0.0, -0.17, -0.16) * scale,
        vec3(0.06, 0.10, 0.06) * scale,
        None,
        darken(color, 0.06),
    );
    // Third segment and a tip curling forward.
    draw_cube(
        head + vec3(0.0, -0.24, -0.205) * scale,
        vec3(0.055, 0.08, 0.055) * scale,
        None,
        color,
    );
    draw_cube(
        head + vec3(0.0, -0.275, -0.25) * scale,
        vec3(0.05, 0.04, 0.06) * scale,
        None,
        brighten(color, 0.06),
    );

    // Cream tusks flanking the trunk.
    let tusk = Color::new(0.94, 0.90, 0.80, 1.0);
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            head + vec3(side * 0.075, -0.12, -0.115) * scale,
            vec3(0.035, 0.09, 0.035) * scale,
            None,
            tusk,
        );
    }

    // Soft inner-ear patches.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            head + vec3(side * 0.18, 0.02, -0.048) * scale,
            vec3(0.042, 0.12, 0.018) * scale,
            None,
            brighten(color, 0.14),
        );
    }

    // Tail with a dark tuft.
    draw_cube(
        center + vec3(0.0, 0.06, 0.29) * scale,
        vec3(0.03, 0.12, 0.03) * scale,
        None,
        darken(color, 0.06),
    );
    draw_toy_sphere(
        center + vec3(0.0, -0.01, 0.29) * scale,
        0.032 * scale,
        None,
        darken(color, 0.18),
    );

    // Toenails on the front feet.
    for x in [-0.12_f32, 0.12] {
        for dx in [-0.026_f32, 0.026] {
            draw_cube(
                center + vec3(x + dx, -0.185, -0.132) * scale,
                vec3(0.026, 0.032, 0.012) * scale,
                None,
                tusk,
            );
        }
    }

    draw_eye_pair(center, 0.26, -0.28, 0.07, scale);
}
