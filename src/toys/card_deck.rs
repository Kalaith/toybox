use super::library::{darken, draw_cube_with_edges, draw_game_box};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let card = Color::new(0.95, 0.93, 0.88, 1.0);

    draw_game_box(center, color, scale);

    // A staggered fan of cards spilling across the lid.
    for index in 0..4 {
        let t = index as f32;
        draw_cube_with_edges(
            center + vec3(-0.12 + t * 0.075, 0.125 + t * 0.012, -0.02 + t * 0.045) * scale,
            vec3(0.11, 0.012, 0.16) * scale,
            card,
        );
    }
    // Face-up top card with a colored pip.
    draw_cube(
        center + vec3(0.12, 0.175, 0.12) * scale,
        vec3(0.04, 0.012, 0.05) * scale,
        None,
        darken(color, 0.20),
    );
    // The rest of the deck stacked in the corner.
    draw_cube_with_edges(
        center + vec3(-0.16, 0.15, 0.10) * scale,
        vec3(0.11, 0.06, 0.15) * scale,
        card,
    );
}
