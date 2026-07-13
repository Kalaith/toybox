use super::library::draw_toy_sphere;
use super::library::{brighten, darken, draw_dragon_base};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_dragon_base(center, color, scale);

    // The namesake tail: seven segments sweeping in a serpentine
    // S-curve, tapering as they go, with gold ridge spikes on top.
    let gold = Color::new(0.96, 0.84, 0.36, 1.0);
    let mut tip = vec3(0.0, 0.02, 0.27);
    for index in 0..7 {
        let t = index as f32;
        let segment = vec3((t * 0.9).sin() * 0.09, 0.02 - t * 0.008, 0.27 + t * 0.115);
        let radius = 0.09 - t * 0.010;
        let tone = if index % 2 == 0 {
            darken(color, 0.05)
        } else {
            brighten(color, 0.04)
        };
        draw_toy_sphere(center + segment * scale, radius * scale, None, tone);
        if index % 2 == 0 && index < 6 {
            draw_cube(
                center + (segment + vec3(0.0, radius * 0.85, 0.0)) * scale,
                vec3(0.028, 0.05, 0.028) * scale,
                None,
                gold,
            );
        }
        tip = segment;
    }

    // Spade at the very end.
    let spade = tip + vec3((6.6_f32 * 0.9).sin() * 0.02, 0.0, 0.09);
    draw_toy_sphere(center + spade * scale, 0.038 * scale, None, gold);
    draw_cube(
        center + (spade + vec3(0.0, 0.0, 0.05)) * scale,
        vec3(0.07, 0.018, 0.045) * scale,
        None,
        gold,
    );
    draw_cube(
        center + (spade + vec3(0.0, 0.0, 0.07)) * scale,
        vec3(0.04, 0.018, 0.07) * scale,
        None,
        gold,
    );
}
