use super::primitives::{darken, draw_cube_with_edges, draw_dragon_base, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);
    let spike = Color::new(0.96, 0.88, 0.58, 1.0);

    // Main crest rises toward the middle, tall spikes get tapered tips.
    let heights = [0.08_f32, 0.11, 0.14, 0.11, 0.08];
    for (index, height) in heights.into_iter().enumerate() {
        let z = -0.24 + index as f32 * 0.11;
        draw_cube_with_edges(
            center + vec3(0.0, 0.26 + height * 0.5, z) * scale,
            vec3(0.07, height, 0.07) * scale,
            spike,
        );
        if height > 0.10 {
            draw_cube(
                center + vec3(0.0, 0.26 + height + 0.022, z) * scale,
                vec3(0.035, 0.05, 0.035) * scale,
                None,
                spike,
            );
        }
    }

    // Smaller spikes flanking the crest.
    for side in [-1.0_f32, 1.0] {
        for z in [-0.13_f32, 0.05] {
            draw_cube(
                center + vec3(side * 0.10, 0.29, z) * scale,
                vec3(0.04, 0.06, 0.04) * scale,
                None,
                spike,
            );
        }
    }

    // Tail ending in a spiked club.
    draw_toy_sphere(
        center + vec3(0.0, 0.05, 0.31) * scale,
        0.075 * scale,
        None,
        darken(color, 0.05),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.08, 0.43) * scale,
        0.060 * scale,
        None,
        color,
    );
    draw_cube(
        center + vec3(0.0, 0.15, 0.43) * scale,
        vec3(0.030, 0.060, 0.030) * scale,
        None,
        spike,
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.075, 0.08, 0.43) * scale,
            vec3(0.050, 0.030, 0.030) * scale,
            None,
            spike,
        );
    }
}
