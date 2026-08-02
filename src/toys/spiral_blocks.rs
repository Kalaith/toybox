use super::primitives::{
    darken, draw_cube_with_edges, draw_studded_block, draw_toy_sphere, shift_block_color,
};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Base plate and central axle the helix winds around.
    draw_cube_with_edges(
        center + vec3(0.0, -0.125, 0.0) * scale,
        vec3(0.34, 0.05, 0.34) * scale,
        darken(color, 0.12),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.21, 0.0) * scale,
        vec3(0.045, 0.72, 0.045) * scale,
        Color::new(0.52, 0.38, 0.26, 1.0),
    );

    // Helix tower: each block steps up, rotates around the axle, and
    // tapers slightly so the spiral narrows as it climbs.
    for index in 0..6 {
        let angle = index as f32 * 0.9;
        let radius = 0.13;
        let width = 0.17 - index as f32 * 0.010;
        draw_studded_block(
            center
                + vec3(
                    angle.cos() * radius,
                    -0.04 + index as f32 * 0.11,
                    angle.sin() * radius,
                ) * scale,
            vec3(width, 0.11, width) * scale,
            shift_block_color(color, index),
        );
    }

    // Gold finial capping the axle.
    draw_toy_sphere(
        center + vec3(0.0, 0.615, 0.0) * scale,
        0.050 * scale,
        None,
        Color::new(0.93, 0.76, 0.32, 1.0),
    );
}
