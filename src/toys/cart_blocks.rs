use super::library::{
    brighten, darken, draw_cube_with_edges, draw_studded_block, draw_toy_sphere, draw_wheel,
    shift_block_color,
};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let wheel = Color::new(0.22, 0.22, 0.24, 1.0);

    // Rolling block cart: flat chassis on four wheels, cargo blocks on top.
    draw_studded_block(
        center + vec3(0.0, 0.10, 0.0) * scale,
        vec3(0.42, 0.10, 0.26) * scale,
        color,
    );
    for x in [-0.15_f32, 0.15] {
        for z in [-0.14_f32, 0.14] {
            draw_wheel(
                center + vec3(x, 0.02, z) * scale,
                0.055 * scale,
                0.04 * scale,
                wheel,
            );
        }
    }

    draw_studded_block(
        center + vec3(-0.09, 0.22, 0.0) * scale,
        vec3(0.16, 0.14, 0.18) * scale,
        shift_block_color(color, 1),
    );
    draw_studded_block(
        center + vec3(0.12, 0.20, 0.0) * scale,
        vec3(0.12, 0.10, 0.14) * scale,
        shift_block_color(color, 2),
    );
    draw_studded_block(
        center + vec3(-0.09, 0.33, 0.0) * scale,
        vec3(0.10, 0.09, 0.12) * scale,
        shift_block_color(color, 3),
    );

    // Striped ball riding in the spare corner of the bed.
    draw_toy_sphere(
        center + vec3(0.12, 0.30, 0.0) * scale,
        0.052 * scale,
        None,
        brighten(color, 0.22),
    );
    draw_cube(
        center + vec3(0.12, 0.30, 0.0) * scale,
        vec3(0.108, 0.018, 0.108) * scale,
        None,
        Color::new(0.92, 0.86, 0.70, 1.0),
    );

    // Low stake rails around the bed.
    for z in [-0.12_f32, 0.12] {
        draw_cube_with_edges(
            center + vec3(0.0, 0.175, z) * scale,
            vec3(0.40, 0.055, 0.020) * scale,
            darken(color, 0.10),
        );
    }
    for x in [-0.20_f32, 0.20] {
        draw_cube_with_edges(
            center + vec3(x, 0.175, 0.0) * scale,
            vec3(0.020, 0.055, 0.24) * scale,
            darken(color, 0.10),
        );
    }

    // Pull handle with a T-grip out front.
    let handle = Color::new(0.42, 0.30, 0.22, 1.0);
    draw_cube_with_edges(
        center + vec3(-0.285, 0.085, 0.0) * scale,
        vec3(0.15, 0.024, 0.024) * scale,
        handle,
    );
    draw_cube_with_edges(
        center + vec3(-0.365, 0.085, 0.0) * scale,
        vec3(0.024, 0.024, 0.11) * scale,
        handle,
    );
}
