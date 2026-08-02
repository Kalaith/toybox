use super::primitives::draw_game_box;
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);
    let tile_face = Color::new(0.96, 0.90, 0.72, 1.0);
    let ink = Color::new(0.30, 0.20, 0.10, 1.0);

    // Six tiles on the lid, each stamped with its own glyph and a
    // little score dot in the corner, Scrabble-style.
    for row in 0..2 {
        for column in 0..3 {
            let tile = vec3(
                -0.15 + column as f32 * 0.15,
                0.14,
                -0.08 + row as f32 * 0.12,
            );
            draw_cube(
                center + tile * scale,
                vec3(0.09, 0.035, 0.08) * scale,
                None,
                tile_face,
            );
            let top = tile + vec3(0.0, 0.022, 0.0);
            // Glyph bars vary per tile: |, -, L, T, +, or dots.
            let index = row * 3 + column;
            let vertical = vec3(0.012, 0.008, 0.042);
            let horizontal = vec3(0.042, 0.008, 0.012);
            match index {
                0 => draw_cube(center + top * scale, vertical * scale, None, ink),
                1 => draw_cube(center + top * scale, horizontal * scale, None, ink),
                2 => {
                    draw_cube(
                        center + (top + vec3(-0.012, 0.0, 0.0)) * scale,
                        vertical * scale,
                        None,
                        ink,
                    );
                    draw_cube(
                        center + (top + vec3(0.008, 0.0, 0.018)) * scale,
                        vec3(0.028, 0.008, 0.012) * scale,
                        None,
                        ink,
                    );
                }
                3 => {
                    draw_cube(
                        center + (top + vec3(0.0, 0.0, -0.016)) * scale,
                        horizontal * scale,
                        None,
                        ink,
                    );
                    draw_cube(
                        center + (top + vec3(0.0, 0.0, 0.008)) * scale,
                        vec3(0.012, 0.008, 0.028) * scale,
                        None,
                        ink,
                    );
                }
                4 => {
                    draw_cube(center + top * scale, vertical * scale, None, ink);
                    draw_cube(center + top * scale, horizontal * scale, None, ink);
                }
                _ => {
                    for dz in [-0.016_f32, 0.016] {
                        draw_cube(
                            center + (top + vec3(0.0, 0.0, dz)) * scale,
                            vec3(0.014, 0.008, 0.014) * scale,
                            None,
                            ink,
                        );
                    }
                }
            }
            // Score dot in the tile corner.
            draw_cube(
                center + (top + vec3(0.030, -0.002, 0.026)) * scale,
                vec3(0.010, 0.008, 0.010) * scale,
                None,
                ink,
            );
        }
    }

    // Wooden rack along the lid front with two tiles standing in it.
    draw_cube(
        center + vec3(0.0, 0.115, -0.145) * scale,
        vec3(0.30, 0.030, 0.035) * scale,
        None,
        Color::new(0.52, 0.38, 0.26, 1.0),
    );
    for x in [-0.06_f32, 0.05] {
        draw_cube(
            center + vec3(x, 0.155, -0.152) * scale,
            vec3(0.08, 0.08, 0.020) * scale,
            None,
            tile_face,
        );
        draw_cube(
            center + vec3(x, 0.158, -0.163) * scale,
            vec3(0.012, 0.040, 0.008) * scale,
            None,
            ink,
        );
    }
}
