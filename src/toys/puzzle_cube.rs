use super::library::{darken, draw_cube_with_edges};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    // One big twisty puzzle cube with a 3x3 facet grid on two faces.
    let body = darken(color, 0.30);
    draw_cube_with_edges(
        center + vec3(0.0, 0.16, 0.0) * scale,
        vec3(0.30, 0.30, 0.30) * scale,
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
    for row in 0..3 {
        for column in 0..3 {
            let facet = facets[(row * 3 + column + 1) % facets.len()];
            // Front face tiles.
            draw_cube(
                center
                    + vec3(
                        -0.09 + column as f32 * 0.09,
                        0.07 + row as f32 * 0.09,
                        -0.155,
                    ) * scale,
                vec3(0.075, 0.075, 0.015) * scale,
                None,
                facet,
            );
            // Top face tiles, offset pattern so the faces read scrambled.
            let top = facets[(row * 2 + column * 3 + 4) % facets.len()];
            draw_cube(
                center
                    + vec3(
                        -0.09 + column as f32 * 0.09,
                        0.315,
                        -0.09 + row as f32 * 0.09,
                    ) * scale,
                vec3(0.075, 0.015, 0.075) * scale,
                None,
                top,
            );
        }
    }
}
