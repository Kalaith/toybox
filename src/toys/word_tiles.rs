use super::library::draw_game_box;
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);
    for row in 0..2 {
        for column in 0..3 {
            draw_cube(
                center
                    + vec3(
                        -0.15 + column as f32 * 0.15,
                        0.14,
                        -0.08 + row as f32 * 0.12,
                    ) * scale,
                vec3(0.09, 0.035, 0.08) * scale,
                None,
                Color::new(0.96, 0.90, 0.72, 1.0),
            );
        }
    }
}
