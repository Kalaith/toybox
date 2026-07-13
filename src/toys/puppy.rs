use super::library::draw_toy_sphere;
use super::library::{brighten, darken, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_toy_sphere(center, 0.25 * scale, None, color);
    draw_toy_sphere(
        center + vec3(0.0, -0.03, -0.15) * scale,
        0.12 * scale,
        None,
        Color::new(0.92, 0.78, 0.60, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.22, -0.15) * scale,
        0.16 * scale,
        None,
        brighten(color, 0.05),
    );
    draw_cube(
        center + vec3(-0.18, 0.20, -0.14) * scale,
        vec3(0.09, 0.24, 0.08) * scale,
        None,
        darken(color, 0.10),
    );
    draw_cube(
        center + vec3(0.18, 0.20, -0.14) * scale,
        vec3(0.09, 0.24, 0.08) * scale,
        None,
        darken(color, 0.10),
    );
    draw_cube(
        center + vec3(0.0, 0.07, -0.24) * scale,
        vec3(0.24, 0.045, 0.040) * scale,
        None,
        Color::new(0.80, 0.18, 0.16, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.11, 0.08, -0.26) * scale,
        0.022 * scale,
        None,
        Color::new(0.96, 0.78, 0.28, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.16, -0.29) * scale,
        0.08 * scale,
        None,
        Color::new(0.92, 0.78, 0.60, 1.0),
    );
    // Wagging tail raised in two segments with a bright tip.
    draw_cube(
        center + vec3(0.0, 0.10, 0.24) * scale,
        vec3(0.06, 0.06, 0.10) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.0, 0.17, 0.29) * scale,
        vec3(0.05, 0.05, 0.08) * scale,
        None,
        darken(color, 0.05),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.225, 0.31) * scale,
        0.035 * scale,
        None,
        brighten(color, 0.12),
    );

    // Front paws and hind paws.
    for x in [-0.14_f32, 0.14] {
        draw_toy_sphere(
            center + vec3(x, -0.16, -0.12) * scale,
            0.055 * scale,
            None,
            brighten(color, 0.04),
        );
        draw_toy_sphere(
            center + vec3(x, -0.17, 0.10) * scale,
            0.060 * scale,
            None,
            brighten(color, 0.04),
        );
    }

    // Darker spot over one eye and another on the back.
    draw_toy_sphere(
        center + vec3(0.07, 0.25, -0.275) * scale,
        0.050 * scale,
        None,
        darken(color, 0.12),
    );
    draw_toy_sphere(
        center + vec3(-0.10, 0.20, 0.06) * scale,
        0.065 * scale,
        None,
        darken(color, 0.10),
    );

    draw_face(center, 0.23, -0.32, 0.07, scale);

    // Happy tongue hanging out of the muzzle.
    draw_cube(
        center + vec3(0.025, 0.09, -0.345) * scale,
        vec3(0.035, 0.055, 0.014) * scale,
        None,
        Color::new(0.94, 0.52, 0.56, 1.0),
    );
}
