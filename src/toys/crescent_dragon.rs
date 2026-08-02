use super::primitives::draw_toy_sphere;
use super::primitives::{brighten, draw_cube_with_edges, draw_dragon_base};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);
    let horn = Color::new(0.96, 0.88, 0.58, 1.0);
    draw_cube_with_edges(
        center + vec3(-0.10, 0.36, -0.34) * scale,
        vec3(0.06, 0.18, 0.06) * scale,
        horn,
    );
    draw_cube_with_edges(
        center + vec3(0.10, 0.36, -0.34) * scale,
        vec3(0.06, 0.18, 0.06) * scale,
        horn,
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.05, 0.34) * scale,
        0.08 * scale,
        None,
        brighten(color, 0.12),
    );
    draw_toy_sphere(
        center + vec3(0.06, 0.08, 0.42) * scale,
        0.055 * scale,
        None,
        horn,
    );

    // Its namesake: a crescent moon cradled between the horns, built
    // from an arc of spheres tapering at the tips, open to the right.
    let moon = Color::new(0.97, 0.90, 0.62, 1.0);
    let arc = [
        (0.0_f32, 0.095, 0.024),
        (-0.080, 0.048, 0.034),
        (-0.080, -0.048, 0.034),
        (0.0, -0.095, 0.024),
    ];
    for (x, y, radius) in arc {
        draw_toy_sphere(
            center + vec3(x, 0.56 + y, -0.34) * scale,
            radius * scale,
            None,
            moon,
        );
    }
    // Tiny star in the crescent's hollow.
    draw_toy_sphere(
        center + vec3(0.045, 0.56, -0.34) * scale,
        0.020 * scale,
        None,
        Color::new(0.98, 0.96, 0.84, 1.0),
    );
}
