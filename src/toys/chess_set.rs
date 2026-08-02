use super::primitives::{darken, draw_cube_with_edges, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let light = Color::new(0.90, 0.86, 0.76, 1.0);
    let dark = darken(color, 0.22);

    // Flat board with a 4x4 checker inlay.
    draw_cube_with_edges(
        center + vec3(0.0, 0.05, 0.0) * scale,
        vec3(0.42, 0.06, 0.42) * scale,
        color,
    );
    for row in 0..4 {
        for column in 0..4 {
            let tile = if (row + column) % 2 == 0 { light } else { dark };
            draw_cube(
                center
                    + vec3(
                        -0.135 + column as f32 * 0.09,
                        0.085,
                        -0.135 + row as f32 * 0.09,
                    ) * scale,
                vec3(0.085, 0.012, 0.085) * scale,
                None,
                tile,
            );
        }
    }

    // A few pieces still standing: pawns as ball-top pegs, one tall king.
    for (x, z, piece) in [
        (-0.10, -0.10, light),
        (0.08, 0.02, dark),
        (0.12, -0.12, light),
    ] {
        draw_cube_with_edges(
            center + vec3(x, 0.14, z) * scale,
            vec3(0.035, 0.09, 0.035) * scale,
            piece,
        );
        draw_toy_sphere(
            center + vec3(x, 0.20, z) * scale,
            0.028 * scale,
            None,
            piece,
        );
    }
    draw_cube_with_edges(
        center + vec3(-0.04, 0.17, 0.10) * scale,
        vec3(0.04, 0.16, 0.04) * scale,
        dark,
    );
    // Full cross on the king.
    draw_cube(
        center + vec3(-0.04, 0.27, 0.10) * scale,
        vec3(0.06, 0.02, 0.02) * scale,
        None,
        dark,
    );
    draw_cube(
        center + vec3(-0.04, 0.275, 0.10) * scale,
        vec3(0.018, 0.055, 0.018) * scale,
        None,
        dark,
    );

    // Light rook with merlon nubs.
    draw_cube_with_edges(
        center + vec3(0.14, 0.145, 0.10) * scale,
        vec3(0.05, 0.10, 0.05) * scale,
        light,
    );
    for x in [-0.017_f32, 0.017] {
        draw_cube(
            center + vec3(0.14 + x, 0.21, 0.10) * scale,
            vec3(0.018, 0.032, 0.018) * scale,
            None,
            light,
        );
    }

    // Dark knight, muzzle jutting forward.
    draw_cube_with_edges(
        center + vec3(-0.14, 0.145, 0.04) * scale,
        vec3(0.04, 0.10, 0.04) * scale,
        dark,
    );
    draw_cube(
        center + vec3(-0.14, 0.21, 0.015) * scale,
        vec3(0.036, 0.045, 0.065) * scale,
        None,
        dark,
    );

    // Captured pawn lying on its side by the board edge.
    draw_cube_with_edges(
        center + vec3(0.0, 0.105, 0.16) * scale,
        vec3(0.09, 0.035, 0.035) * scale,
        light,
    );
    draw_toy_sphere(
        center + vec3(0.058, 0.105, 0.16) * scale,
        0.028 * scale,
        None,
        light,
    );
}
