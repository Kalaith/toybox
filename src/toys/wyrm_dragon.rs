use super::library::{brighten, darken, draw_eye_pair, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Wingless serpent: body segments winding in an S-curve, head raised.
    let segments = [
        (vec3(0.0, 0.26, -0.26), 0.13),
        (vec3(0.10, 0.14, -0.12), 0.14),
        (vec3(0.0, 0.10, 0.04), 0.145),
        (vec3(-0.12, 0.08, 0.20), 0.13),
        (vec3(-0.02, 0.06, 0.35), 0.10),
        (vec3(0.10, 0.05, 0.44), 0.07),
    ];
    for (index, (offset, radius)) in segments.iter().enumerate() {
        let tone = if index % 2 == 0 {
            color
        } else {
            brighten(color, 0.08)
        };
        draw_toy_sphere(center + *offset * scale, radius * scale, None, tone);
    }

    // Back ridge fins along the spine.
    for (offset, _) in segments.iter().take(4) {
        draw_cube(
            center + (*offset + vec3(0.0, 0.13, 0.0)) * scale,
            vec3(0.035, 0.09, 0.06) * scale,
            None,
            darken(color, 0.14),
        );
    }

    // Pale belly scutes down the coil's front.
    let scute = Color::new(0.90, 0.76, 0.54, 1.0);
    for (offset, radius) in segments.iter().skip(1).take(3) {
        draw_cube(
            center + (*offset + vec3(0.0, -radius * 0.78, -0.02)) * scale,
            vec3(0.09, 0.035, 0.07) * scale,
            None,
            scute,
        );
    }

    // Snout with drooping barbel whiskers, eastern-dragon style.
    draw_toy_sphere(
        center + vec3(0.0, 0.24, -0.375) * scale,
        0.055 * scale,
        None,
        brighten(color, 0.14),
    );
    let barbel = Color::new(0.94, 0.86, 0.60, 1.0);
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.055, 0.235, -0.415) * scale,
            vec3(0.012, 0.012, 0.070) * scale,
            None,
            barbel,
        );
        draw_cube(
            center + vec3(side * 0.060, 0.20, -0.445) * scale,
            vec3(0.012, 0.055, 0.012) * scale,
            None,
            barbel,
        );
    }

    // Horns swept back over the crest.
    let horn = Color::new(0.96, 0.88, 0.58, 1.0);
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.07, 0.355, -0.20) * scale,
            vec3(0.030, 0.030, 0.10) * scale,
            None,
            horn,
        );
    }

    // Gold tip and tuft ending the tail.
    draw_toy_sphere(
        center + vec3(0.16, 0.05, 0.50) * scale,
        0.040 * scale,
        None,
        horn,
    );
    draw_cube(
        center + vec3(0.16, 0.10, 0.50) * scale,
        vec3(0.025, 0.060, 0.025) * scale,
        None,
        horn,
    );

    draw_eye_pair(center, 0.30, -0.37, 0.06, scale);
}
