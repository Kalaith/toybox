use super::primitives::{brighten, draw_game_box, draw_toy_sphere};
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);

    // Cream dial plate under the wheel.
    let plate = Color::new(0.93, 0.90, 0.82, 1.0);
    draw_cube(
        center + vec3(0.0, 0.118, 0.0) * scale,
        vec3(0.36, 0.010, 0.26) * scale,
        None,
        plate,
    );
    draw_cube(
        center + vec3(0.0, 0.118, 0.0) * scale,
        vec3(0.26, 0.010, 0.36) * scale,
        None,
        plate,
    );

    // Eight color wedges fill out the spinner wheel.
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
    let diagonals = [
        Color::new(0.94, 0.58, 0.26, 1.0),
        Color::new(0.24, 0.68, 0.68, 1.0),
        Color::new(0.56, 0.34, 0.72, 1.0),
        Color::new(0.86, 0.36, 0.62, 1.0),
    ];
    for (index, wedge) in diagonals.iter().enumerate() {
        let angle = (index as f32 + 0.5) / diagonals.len() as f32 * std::f32::consts::TAU;
        draw_cube(
            center + vec3(angle.cos() * 0.13, 0.122, angle.sin() * 0.13) * scale,
            vec3(0.075, 0.012, 0.075) * scale,
            None,
            *wedge,
        );
    }

    // Hub, arrow pointer with a wide head, and a gold pivot cap.
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
    draw_cube(
        center + vec3(0.145, 0.151, -0.05) * scale,
        vec3(0.04, 0.016, 0.055) * scale,
        None,
        brighten(color, 0.30),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.168, 0.0) * scale,
        0.018 * scale,
        None,
        Color::new(0.94, 0.76, 0.30, 1.0),
    );

    // Player tokens waiting on the lid corners.
    draw_cube(
        center + vec3(-0.22, 0.115, 0.13) * scale,
        vec3(0.045, 0.014, 0.045) * scale,
        None,
        Color::new(0.82, 0.24, 0.20, 1.0),
    );
    draw_cube(
        center + vec3(-0.20, 0.115, -0.13) * scale,
        vec3(0.045, 0.014, 0.045) * scale,
        None,
        Color::new(0.26, 0.46, 0.86, 1.0),
    );
}
