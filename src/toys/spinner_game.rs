use super::library::{brighten, draw_game_box, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);

    // Color wedges around the spinner dial.
    let wedges = [
        Color::new(0.90, 0.36, 0.30, 1.0),
        Color::new(0.94, 0.78, 0.28, 1.0),
        Color::new(0.42, 0.78, 0.46, 1.0),
        Color::new(0.40, 0.62, 0.90, 1.0),
    ];
    for (index, wedge) in wedges.iter().enumerate() {
        let angle = index as f32 / wedges.len() as f32 * std::f32::consts::TAU;
        draw_cube(
            center + vec3(angle.cos() * 0.13, 0.125, angle.sin() * 0.13) * scale,
            vec3(0.09, 0.012, 0.09) * scale,
            None,
            *wedge,
        );
    }

    // Hub and pointer arrow.
    draw_toy_sphere(
        center + vec3(0.0, 0.15, 0.0) * scale,
        0.035 * scale,
        None,
        Color::new(0.12, 0.12, 0.13, 1.0),
    );
    draw_cube(
        center + vec3(0.07, 0.15, -0.05) * scale,
        vec3(0.14, 0.015, 0.03) * scale,
        None,
        brighten(color, 0.30),
    );
}
