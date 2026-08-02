use super::primitives::{
    darken, draw_cube_with_edges, draw_robot_arms, draw_robot_core, draw_toy_sphere,
};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);
    // Silver bezel framing the face screen.
    draw_cube(
        center + vec3(0.0, 0.39, -0.145) * scale,
        vec3(0.24, 0.17, 0.02) * scale,
        None,
        Color::new(0.80, 0.84, 0.84, 1.0),
    );
    draw_cube(
        center + vec3(0.0, 0.39, -0.15) * scale,
        vec3(0.21, 0.14, 0.035) * scale,
        None,
        Color::new(0.08, 0.13, 0.17, 1.0),
    );
    let glow = Color::new(0.46, 0.95, 0.92, 1.0);
    draw_cube(
        center + vec3(0.0, 0.40, -0.19) * scale,
        vec3(0.13, 0.020, 0.014) * scale,
        None,
        Color::new(0.18, 0.42, 0.46, 1.0),
    );
    draw_cube(
        center + vec3(-0.05, 0.40, -0.18) * scale,
        vec3(0.04, 0.035, 0.025) * scale,
        None,
        glow,
    );
    draw_cube(
        center + vec3(0.05, 0.40, -0.18) * scale,
        vec3(0.04, 0.035, 0.025) * scale,
        None,
        glow,
    );
    // Pixel smile under the eyes.
    let smile = [(-0.045_f32, 0.355_f32), (0.0, 0.345), (0.045, 0.355)];
    for (x, y) in smile {
        draw_cube(
            center + vec3(x, y, -0.18) * scale,
            vec3(0.025, 0.020, 0.020) * scale,
            None,
            glow,
        );
    }
    // Power LED and tuning knob below the screen.
    draw_cube(
        center + vec3(0.06, 0.295, -0.145) * scale,
        vec3(0.014, 0.014, 0.012) * scale,
        None,
        Color::new(0.92, 0.30, 0.24, 1.0),
    );
    draw_toy_sphere(
        center + vec3(-0.06, 0.295, -0.145) * scale,
        0.016 * scale,
        None,
        Color::new(0.80, 0.84, 0.84, 1.0),
    );
    // Rabbit-ear antennae with ball tips.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.06, 0.56, 0.0) * scale,
            vec3(0.016, 0.10, 0.016) * scale,
            None,
            Color::new(0.80, 0.84, 0.84, 1.0),
        );
        draw_toy_sphere(
            center + vec3(side * 0.06, 0.625, 0.0) * scale,
            0.020 * scale,
            None,
            Color::new(0.80, 0.84, 0.84, 1.0),
        );
    }
    draw_robot_arms(center, color, scale);

    // TV-stand pedestal instead of feet.
    draw_cube_with_edges(
        center + vec3(0.0, -0.135, 0.0) * scale,
        vec3(0.07, 0.07, 0.07) * scale,
        darken(color, 0.12),
    );
    draw_cube_with_edges(
        center + vec3(0.0, -0.19, 0.0) * scale,
        vec3(0.26, 0.045, 0.20) * scale,
        darken(color, 0.16),
    );
}
