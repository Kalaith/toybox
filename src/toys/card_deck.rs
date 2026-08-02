use super::primitives::{darken, draw_cube_with_edges, draw_game_box};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let card = Color::new(0.95, 0.93, 0.88, 1.0);

    draw_game_box(center, color, scale);

    // A staggered fan of cards spilling across the lid, suit pips
    // alternating red and black on each exposed corner.
    let red_pip = Color::new(0.78, 0.20, 0.18, 1.0);
    let black_pip = Color::new(0.12, 0.12, 0.14, 1.0);
    for index in 0..4 {
        let t = index as f32;
        let card_center =
            center + vec3(-0.12 + t * 0.075, 0.125 + t * 0.012, -0.02 + t * 0.045) * scale;
        draw_cube_with_edges(card_center, vec3(0.11, 0.012, 0.16) * scale, card);
        draw_cube(
            card_center + vec3(-0.032, 0.010, -0.052) * scale,
            vec3(0.026, 0.008, 0.032) * scale,
            None,
            if index % 2 == 0 { red_pip } else { black_pip },
        );
    }
    // Face-up top card with a colored pip.
    draw_cube(
        center + vec3(0.12, 0.175, 0.12) * scale,
        vec3(0.04, 0.012, 0.05) * scale,
        None,
        darken(color, 0.20),
    );
    // The rest of the deck stacked in the corner, wearing a patterned
    // back and a gold wrap band.
    draw_cube_with_edges(
        center + vec3(-0.16, 0.15, 0.10) * scale,
        vec3(0.11, 0.06, 0.15) * scale,
        card,
    );
    draw_cube(
        center + vec3(-0.16, 0.182, 0.10) * scale,
        vec3(0.085, 0.008, 0.12) * scale,
        None,
        darken(color, 0.05),
    );
    draw_cube(
        center + vec3(-0.16, 0.15, 0.10) * scale,
        vec3(0.114, 0.064, 0.042) * scale,
        None,
        Color::new(0.93, 0.76, 0.32, 1.0),
    );
}
