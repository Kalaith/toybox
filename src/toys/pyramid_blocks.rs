use super::primitives::{darken, draw_cube_with_edges, draw_studded_block, shift_block_color};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Stepped pyramid: three shrinking tiers under a gold capstone.
    let tiers = [(0.0_f32, 0.40_f32), (0.15, 0.28), (0.30, 0.16)];
    for (index, (height, width)) in tiers.iter().enumerate() {
        draw_studded_block(
            center + vec3(0.0, -0.02 + height, 0.0) * scale,
            vec3(*width, 0.15, *width) * scale,
            shift_block_color(color, index),
        );
    }
    let gold = Color::new(0.93, 0.76, 0.32, 1.0);
    draw_cube_with_edges(
        center + vec3(0.0, 0.405, 0.0) * scale,
        vec3(0.08, 0.06, 0.08) * scale,
        gold,
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.455, 0.0) * scale,
        vec3(0.045, 0.045, 0.045) * scale,
        gold,
    );

    // Dark entrance at the base and steps climbing the front tiers.
    draw_cube(
        center + vec3(0.0, -0.01, -0.202) * scale,
        vec3(0.07, 0.11, 0.015) * scale,
        None,
        Color::new(0.16, 0.13, 0.10, 1.0),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.075, -0.17) * scale,
        vec3(0.09, 0.04, 0.06) * scale,
        darken(color, 0.10),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.225, -0.11) * scale,
        vec3(0.09, 0.04, 0.06) * scale,
        darken(color, 0.10),
    );

    // Palm tree keeping the pyramid company.
    draw_cube_with_edges(
        center + vec3(0.27, 0.005, -0.10) * scale,
        vec3(0.03, 0.16, 0.03) * scale,
        Color::new(0.52, 0.38, 0.26, 1.0),
    );
    let frond = Color::new(0.30, 0.62, 0.34, 1.0);
    draw_cube(
        center + vec3(0.27, 0.095, -0.10) * scale,
        vec3(0.15, 0.015, 0.05) * scale,
        None,
        frond,
    );
    draw_cube(
        center + vec3(0.27, 0.095, -0.10) * scale,
        vec3(0.05, 0.015, 0.15) * scale,
        None,
        frond,
    );
}
