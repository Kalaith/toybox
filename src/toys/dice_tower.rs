use super::primitives::{brighten, darken, draw_cube_with_edges};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    let pip = Color::new(0.95, 0.94, 0.90, 1.0);

    // Tall dice-rolling tower with a chute mouth at the bottom.
    draw_cube_with_edges(
        center + vec3(-0.08, 0.24, 0.04) * scale,
        vec3(0.20, 0.46, 0.20) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(-0.08, 0.50, 0.04) * scale,
        vec3(0.24, 0.07, 0.24) * scale,
        darken(color, 0.12),
    );
    draw_cube_with_edges(
        center + vec3(-0.08, 0.06, -0.12) * scale,
        vec3(0.16, 0.10, 0.14) * scale,
        brighten(color, 0.10),
    );

    // Drop slot up top and the chute mouth below read as openings.
    let opening = Color::new(0.10, 0.09, 0.10, 1.0);
    draw_cube(
        center + vec3(-0.08, 0.33, -0.062) * scale,
        vec3(0.09, 0.13, 0.012) * scale,
        None,
        opening,
    );
    draw_cube(
        center + vec3(-0.08, 0.055, -0.185) * scale,
        vec3(0.10, 0.06, 0.012) * scale,
        None,
        opening,
    );

    // Landing tray in front of the chute.
    draw_cube_with_edges(
        center + vec3(0.14, 0.015, -0.08) * scale,
        vec3(0.32, 0.030, 0.30) * scale,
        darken(color, 0.18),
    );

    // Two dice spilled onto the tray showing a three and a two.
    let ink = Color::new(0.10, 0.09, 0.08, 1.0);
    let dice = [
        (
            vec3(0.14, 0.075, -0.16),
            pip,
            &[(-0.022_f32, -0.022_f32), (0.0, 0.0), (0.022, 0.022)][..],
        ),
        (
            vec3(0.24, 0.075, 0.0),
            brighten(color, 0.25),
            &[(-0.02, -0.02), (0.02, 0.02)][..],
        ),
    ];
    for (offset, die_color, pips) in dice {
        draw_cube_with_edges(
            center + offset * scale,
            vec3(0.09, 0.09, 0.09) * scale,
            die_color,
        );
        for (px, pz) in pips {
            draw_cube(
                center + (offset + vec3(*px, 0.047, *pz)) * scale,
                vec3(0.020, 0.008, 0.020) * scale,
                None,
                ink,
            );
        }
    }
    // Face pips on the white die's front.
    for x in [-0.022_f32, 0.022] {
        draw_cube(
            center + vec3(0.14 + x, 0.075 + x, -0.207) * scale,
            vec3(0.020, 0.020, 0.008) * scale,
            None,
            ink,
        );
    }
}
