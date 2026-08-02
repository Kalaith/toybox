use super::primitives::{brighten, darken, draw_cube_with_edges, draw_toy_sphere, draw_wheel};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_cube_with_edges(
        center + vec3(-0.10, 0.04, 0.0) * scale,
        vec3(0.42, 0.22, 0.22) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.18, 0.14, 0.0) * scale,
        vec3(0.18, 0.20, 0.20) * scale,
        brighten(color, 0.08),
    );

    // Running boards and wheels on BOTH sides.
    let wheel_color = darken(color, 0.22);
    for side in [-1.0_f32, 1.0] {
        draw_cube_with_edges(
            center + vec3(-0.06, -0.10, side * 0.10) * scale,
            vec3(0.54, 0.08, 0.08) * scale,
            wheel_color,
        );
        for offset in [-0.22_f32, 0.16] {
            draw_wheel(
                center + vec3(offset, -0.08, side * 0.15) * scale,
                0.070 * scale,
                0.030 * scale,
                wheel_color,
            );
        }
    }

    // Chimney up front where it belongs, trailing smoke puffs.
    draw_cube(
        center + vec3(-0.24, 0.22, 0.0) * scale,
        vec3(0.035, 0.14, 0.06) * scale,
        None,
        darken(color, 0.12),
    );
    draw_toy_sphere(
        center + vec3(-0.24, 0.33, 0.0) * scale,
        0.035 * scale,
        None,
        Color::new(0.90, 0.90, 0.92, 0.55),
    );
    draw_toy_sphere(
        center + vec3(-0.21, 0.40, 0.02) * scale,
        0.046 * scale,
        None,
        Color::new(0.90, 0.90, 0.92, 0.32),
    );

    // Gold bell on the boiler and a warm headlamp up front.
    draw_toy_sphere(
        center + vec3(-0.05, 0.185, 0.0) * scale,
        0.035 * scale,
        None,
        Color::new(0.93, 0.76, 0.32, 1.0),
    );
    draw_toy_sphere(
        center + vec3(-0.315, 0.08, 0.0) * scale,
        0.030 * scale,
        None,
        Color::new(0.97, 0.88, 0.60, 1.0),
    );

    // Cowcatcher steps at the nose.
    draw_cube_with_edges(
        center + vec3(-0.34, -0.10, 0.0) * scale,
        vec3(0.09, 0.05, 0.20) * scale,
        darken(color, 0.10),
    );
    draw_cube_with_edges(
        center + vec3(-0.38, -0.13, 0.0) * scale,
        vec3(0.06, 0.05, 0.16) * scale,
        darken(color, 0.14),
    );

    // Cab windows: one on each side plus the front lookout.
    let glass = Color::new(0.10, 0.13, 0.16, 1.0);
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(0.18, 0.17, side * 0.102) * scale,
            vec3(0.09, 0.08, 0.012) * scale,
            None,
            glass,
        );
    }
    draw_cube(
        center + vec3(0.088, 0.18, 0.0) * scale,
        vec3(0.012, 0.07, 0.10) * scale,
        None,
        glass,
    );
}
