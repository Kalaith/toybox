use super::primitives::{darken, draw_cube_with_edges};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // Twisty puzzle cube caught mid-turn: the top layer sits shifted
    // off-axis, and all four visible faces carry scrambled tiles.
    let body = darken(color, 0.30);
    let twist = vec3(0.055, 0.0, 0.02);
    draw_cube_with_edges(
        center + vec3(0.0, 0.105, 0.0) * scale,
        vec3(0.30, 0.19, 0.30) * scale,
        body,
    );
    draw_cube_with_edges(
        center + (vec3(0.0, 0.255, 0.0) + twist) * scale,
        vec3(0.30, 0.10, 0.30) * scale,
        body,
    );

    let facets = [
        Color::new(0.90, 0.34, 0.28, 1.0),
        Color::new(0.94, 0.78, 0.28, 1.0),
        Color::new(0.40, 0.76, 0.44, 1.0),
        Color::new(0.38, 0.60, 0.90, 1.0),
        Color::new(0.93, 0.92, 0.88, 1.0),
        Color::new(0.90, 0.56, 0.26, 1.0),
    ];
    let rows = [0.06_f32, 0.15, 0.255];
    for (row, y) in rows.into_iter().enumerate() {
        // The top row rides the twisted layer.
        let shift = if row == 2 { twist } else { Vec3::ZERO };
        for column in 0..3 {
            let lane = -0.09 + column as f32 * 0.09;
            // Front face tiles.
            draw_cube(
                center + (vec3(lane, y, -0.155) + shift) * scale,
                vec3(0.075, 0.075, 0.015) * scale,
                None,
                facets[(row * 3 + column + 1) % facets.len()],
            );
            // Side face tiles, both flanks.
            for side in [-1.0_f32, 1.0] {
                let pick = if side < 0.0 {
                    (row + column * 2 + 3) % facets.len()
                } else {
                    (row * 2 + column + 5) % facets.len()
                };
                draw_cube(
                    center + (vec3(side * 0.155, y, lane) + shift) * scale,
                    vec3(0.015, 0.075, 0.075) * scale,
                    None,
                    facets[pick],
                );
            }
            // Top face tiles follow the twisted layer.
            draw_cube(
                center + (vec3(lane, 0.312, -0.09 + row as f32 * 0.09) + twist) * scale,
                vec3(0.075, 0.015, 0.075) * scale,
                None,
                facets[(row * 2 + column * 3 + 4) % facets.len()],
            );
        }
    }
}
