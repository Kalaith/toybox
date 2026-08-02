use super::primitives::{brighten, darken, draw_robot_core, draw_toy_sphere, draw_wheel};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_robot_core(center, color, scale);

    // A real tripod stance: one leg forward, two trailing, each with a
    // silver hip joint, a bright knee band, and a caster wheel.
    let legs = [(0.0_f32, -0.20_f32), (-0.17, 0.14), (0.17, 0.14)];
    for (x, z) in legs {
        draw_toy_sphere(
            center + vec3(x * 0.85, -0.045, z * 0.85) * scale,
            0.045 * scale,
            None,
            Color::new(0.80, 0.84, 0.84, 1.0),
        );
        draw_cube(
            center + vec3(x, -0.16, z) * scale,
            vec3(0.08, 0.26, 0.08) * scale,
            None,
            darken(color, 0.16),
        );
        draw_cube(
            center + vec3(x, -0.16, z) * scale,
            vec3(0.085, 0.040, 0.085) * scale,
            None,
            brighten(color, 0.08),
        );
        draw_wheel(
            center + vec3(x, -0.30, z) * scale,
            0.050 * scale,
            0.020 * scale,
            Color::new(0.12, 0.13, 0.14, 1.0),
        );
    }

    // Periscope eye raised off the head.
    draw_cube(
        center + vec3(0.0, 0.58, 0.0) * scale,
        vec3(0.030, 0.14, 0.030) * scale,
        None,
        Color::new(0.80, 0.84, 0.84, 1.0),
    );
    draw_cube(
        center + vec3(0.0, 0.665, -0.02) * scale,
        vec3(0.060, 0.050, 0.090) * scale,
        None,
        brighten(color, 0.10),
    );
    draw_cube(
        center + vec3(0.0, 0.665, -0.068) * scale,
        vec3(0.030, 0.030, 0.012) * scale,
        None,
        Color::new(0.42, 0.95, 0.96, 1.0),
    );
}
