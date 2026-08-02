use super::primitives::{brighten, darken, draw_cube_with_edges, draw_robot_arms, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Slim torso balancing on a single ball wheel.
    draw_toy_sphere(
        center + vec3(0.0, 0.02, 0.0) * scale,
        0.14 * scale,
        None,
        Color::new(0.24, 0.26, 0.28, 1.0),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.20, 0.0) * scale,
        vec3(0.16, 0.14, 0.14) * scale,
        darken(color, 0.10),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.36, 0.0) * scale,
        vec3(0.24, 0.22, 0.20) * scale,
        color,
    );
    draw_robot_arms(center + vec3(0.0, 0.10, 0.0) * scale, color, scale * 0.8);

    // Fender steps guarding the ball wheel, front and back.
    let fender = darken(color, 0.18);
    draw_cube(
        center + vec3(0.0, 0.145, -0.09) * scale,
        vec3(0.12, 0.020, 0.060) * scale,
        None,
        fender,
    );
    draw_cube(
        center + vec3(0.0, 0.115, -0.13) * scale,
        vec3(0.12, 0.020, 0.050) * scale,
        None,
        fender,
    );
    draw_cube(
        center + vec3(0.0, 0.145, 0.09) * scale,
        vec3(0.12, 0.020, 0.060) * scale,
        None,
        fender,
    );

    // Stabilizer skirt ringing the waist.
    draw_cube(
        center + vec3(0.0, 0.145, 0.0) * scale,
        vec3(0.28, 0.016, 0.24) * scale,
        None,
        brighten(color, 0.05),
    );

    // Chest screen with a row of status lights.
    draw_cube(
        center + vec3(0.0, 0.39, -0.105) * scale,
        vec3(0.10, 0.055, 0.012) * scale,
        None,
        Color::new(0.08, 0.10, 0.12, 1.0),
    );
    let lights = [
        (-0.055_f32, Color::new(0.42, 0.95, 0.96, 1.0)),
        (0.0, Color::new(0.95, 0.76, 0.30, 1.0)),
        (0.055, Color::new(0.92, 0.30, 0.24, 1.0)),
    ];
    for (x, light) in lights {
        draw_toy_sphere(
            center + vec3(x, 0.325, -0.105) * scale,
            0.014 * scale,
            None,
            light,
        );
    }

    // Narrow visor head.
    draw_cube_with_edges(
        center + vec3(0.0, 0.54, 0.0) * scale,
        vec3(0.18, 0.10, 0.16) * scale,
        brighten(color, 0.08),
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.55, -0.085) * scale,
        vec3(0.13, 0.04, 0.02) * scale,
        Color::new(0.42, 0.95, 0.96, 1.0),
    );

    // Antenna nub on the head.
    draw_cube(
        center + vec3(0.05, 0.615, 0.02) * scale,
        vec3(0.014, 0.045, 0.014) * scale,
        None,
        Color::new(0.80, 0.84, 0.84, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.05, 0.65, 0.02) * scale,
        0.020 * scale,
        None,
        Color::new(0.42, 0.95, 0.96, 1.0),
    );
}
