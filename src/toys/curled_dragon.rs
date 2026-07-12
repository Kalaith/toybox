use super::library::{brighten, darken, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Asleep and curled flat: body spirals inward, head resting on the coil.
    for index in 0..9 {
        let t = index as f32 / 8.0;
        let angle = t * std::f32::consts::TAU * 1.15 + 1.9;
        let radius = 0.26 - t * 0.17;
        let segment_radius = 0.105 - t * 0.045;
        let tone = if index % 2 == 0 {
            color
        } else {
            brighten(color, 0.07)
        };
        draw_toy_sphere(
            center + vec3(angle.cos() * radius, 0.07, angle.sin() * radius) * scale,
            segment_radius * scale,
            None,
            tone,
        );
    }
    // Gold spade tip where the spiral runs out.
    let tip_t = 9.0_f32 / 8.0;
    let tip_angle = tip_t * std::f32::consts::TAU * 1.15 + 1.9;
    let tip_radius = 0.26 - tip_t * 0.17;
    draw_toy_sphere(
        center
            + vec3(
                tip_angle.cos() * tip_radius,
                0.075,
                tip_angle.sin() * tip_radius,
            ) * scale,
        0.045 * scale,
        None,
        Color::new(0.96, 0.88, 0.58, 1.0),
    );

    // Folded wing resting over the outer coil.
    draw_cube(
        center + vec3(0.10, 0.155, 0.05) * scale,
        vec3(0.11, 0.030, 0.17) * scale,
        None,
        darken(color, 0.10),
    );
    draw_cube(
        center + vec3(0.10, 0.175, 0.03) * scale,
        vec3(0.07, 0.022, 0.11) * scale,
        None,
        brighten(color, 0.08),
    );

    // Head resting on top of the outer coil, eyes closed low.
    let head = center + vec3(-0.06, 0.17, -0.22) * scale;
    draw_toy_sphere(head, 0.11 * scale, None, brighten(color, 0.10));
    draw_toy_sphere(
        head + vec3(0.0, -0.01, -0.10) * scale,
        0.05 * scale,
        None,
        brighten(color, 0.16),
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            head + vec3(side * 0.06, 0.10, 0.02) * scale,
            vec3(0.03, 0.06, 0.03) * scale,
            None,
            darken(color, 0.12),
        );
    }
    // Shut eyelids: thin rest lines instead of open eyes.
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.05 - 0.06, 0.19, -0.315) * scale,
            vec3(0.045, 0.012, 0.014) * scale,
            None,
            Color::new(0.035, 0.030, 0.026, 1.0),
        );
    }

    // A drowsy trail of Zs drifting up from the snout.
    let zs = [
        (0.06_f32, 0.32, 0.030, 0.90),
        (0.11, 0.40, 0.024, 0.65),
        (0.15, 0.47, 0.018, 0.42),
    ];
    for (x, y, size, alpha) in zs {
        draw_cube(
            center + vec3(x, y, -0.28) * scale,
            vec3(size, size, 0.012) * scale,
            None,
            Color::new(0.95, 0.92, 0.80, alpha),
        );
    }
}
