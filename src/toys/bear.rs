use super::library::draw_toy_sphere;
use super::library::{brighten, draw_face};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let tan = Color::new(0.93, 0.80, 0.62, 1.0);
    draw_toy_sphere(center, 0.26 * scale, None, color);
    draw_toy_sphere(
        center + vec3(0.0, -0.04, -0.18) * scale,
        0.13 * scale,
        None,
        tan,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.22, -0.12) * scale,
        0.17 * scale,
        None,
        brighten(color, 0.06),
    );
    draw_toy_sphere(
        center + vec3(-0.16, 0.36, -0.12) * scale,
        0.08 * scale,
        None,
        color,
    );
    draw_toy_sphere(
        center + vec3(0.16, 0.36, -0.12) * scale,
        0.08 * scale,
        None,
        color,
    );
    for x in [-0.16_f32, 0.16] {
        draw_toy_sphere(
            center + vec3(x, 0.36, -0.15) * scale,
            0.045 * scale,
            None,
            tan,
        );
        draw_toy_sphere(
            center + vec3(x, -0.16, -0.13) * scale,
            0.065 * scale,
            None,
            brighten(color, 0.05),
        );
    }
    draw_toy_sphere(
        center + vec3(0.0, 0.18, -0.26) * scale,
        0.07 * scale,
        None,
        tan,
    );
    draw_face(center, 0.22, -0.28, 0.07, scale);

    // Stubby arms hugging forward, with tan paw pads.
    for x in [-0.22_f32, 0.22] {
        draw_toy_sphere(
            center + vec3(x, 0.04, -0.10) * scale,
            0.085 * scale,
            None,
            brighten(color, 0.05),
        );
        draw_toy_sphere(
            center + vec3(x * 1.08, 0.02, -0.165) * scale,
            0.045 * scale,
            None,
            tan,
        );
    }

    // Tan pads on the feet fronts.
    for x in [-0.16_f32, 0.16] {
        draw_toy_sphere(
            center + vec3(x, -0.165, -0.185) * scale,
            0.035 * scale,
            None,
            tan,
        );
    }

    // Round tail.
    draw_toy_sphere(
        center + vec3(0.0, -0.06, 0.24) * scale,
        0.07 * scale,
        None,
        brighten(color, 0.04),
    );

    // Cream ribbon bow at the throat.
    let ribbon = Color::new(0.95, 0.92, 0.80, 1.0);
    draw_toy_sphere(
        center + vec3(0.0, 0.10, -0.24) * scale,
        0.035 * scale,
        None,
        ribbon,
    );
    for x in [-0.05_f32, 0.05] {
        draw_cube(
            center + vec3(x, 0.10, -0.235) * scale,
            vec3(0.055, 0.045, 0.022) * scale,
            None,
            ribbon,
        );
    }
}
