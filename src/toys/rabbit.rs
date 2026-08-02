use super::primitives::draw_toy_sphere;
use super::primitives::{brighten, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let inner = Color::new(0.96, 0.72, 0.78, 1.0);
    draw_toy_sphere(center, 0.23 * scale, None, color);
    draw_toy_sphere(
        center + vec3(0.0, -0.02, -0.16) * scale,
        0.12 * scale,
        None,
        Color::new(0.94, 0.90, 0.82, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.22, -0.17) * scale,
        0.15 * scale,
        None,
        brighten(color, 0.06),
    );
    draw_cube(
        center + vec3(-0.08, 0.45, -0.17) * scale,
        vec3(0.07, 0.28, 0.06) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.08, 0.45, -0.17) * scale,
        vec3(0.07, 0.28, 0.06) * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(-0.08, 0.45, -0.205) * scale,
        vec3(0.035, 0.22, 0.018) * scale,
        None,
        inner,
    );
    draw_cube(
        center + vec3(0.08, 0.45, -0.205) * scale,
        vec3(0.035, 0.22, 0.018) * scale,
        None,
        inner,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.01, 0.24) * scale,
        0.07 * scale,
        None,
        Color::new(0.94, 0.90, 0.82, 1.0),
    );
    for x in [-0.13_f32, 0.13] {
        draw_toy_sphere(
            center + vec3(x, -0.16, -0.11) * scale,
            0.055 * scale,
            None,
            brighten(color, 0.06),
        );
    }
    draw_face(center, 0.23, -0.31, 0.06, scale);

    // Buck teeth under the nose.
    for x in [-0.013_f32, 0.013] {
        draw_cube(
            center + vec3(x, 0.16, -0.315) * scale,
            vec3(0.014, 0.028, 0.010) * scale,
            None,
            Color::new(0.96, 0.95, 0.90, 1.0),
        );
    }

    // Whisker dots on the cheeks.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.10, 0.185, -0.30) * scale,
            vec3(0.010, 0.010, 0.008) * scale,
            None,
            Color::new(0.10, 0.09, 0.08, 1.0),
        );
        draw_cube(
            center + vec3(side * 0.115, 0.165, -0.29) * scale,
            vec3(0.010, 0.010, 0.008) * scale,
            None,
            Color::new(0.10, 0.09, 0.08, 1.0),
        );
    }

    // One ear tip flops forward.
    draw_cube(
        center + vec3(0.08, 0.575, -0.215) * scale,
        vec3(0.075, 0.055, 0.055) * scale,
        None,
        color,
    );

    // Big hind feet stretched out front.
    for x in [-0.10_f32, 0.10] {
        draw_cube(
            center + vec3(x, -0.20, -0.13) * scale,
            vec3(0.07, 0.045, 0.15) * scale,
            None,
            brighten(color, 0.06),
        );
    }

    // A carrot held against the chest.
    draw_cube(
        center + vec3(0.05, -0.03, -0.27) * scale,
        vec3(0.045, 0.09, 0.045) * scale,
        None,
        Color::new(0.92, 0.52, 0.20, 1.0),
    );
    draw_cube(
        center + vec3(0.05, -0.095, -0.27) * scale,
        vec3(0.028, 0.05, 0.028) * scale,
        None,
        Color::new(0.86, 0.44, 0.16, 1.0),
    );
    for (x, y) in [(0.035_f32, 0.045_f32), (0.065, 0.04)] {
        draw_cube(
            center + vec3(x, y, -0.27) * scale,
            vec3(0.016, 0.05, 0.016) * scale,
            None,
            Color::new(0.34, 0.62, 0.30, 1.0),
        );
    }
}
