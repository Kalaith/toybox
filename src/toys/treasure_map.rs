use super::primitives::draw_game_box;
use macroquad::prelude::*;

pub fn draw(center: Vec3, color: Color, scale: f32) {
    draw_game_box(center, color, scale);

    // Parchment sheet with a rolled right edge.
    draw_cube(
        center + vec3(0.0, 0.14, -0.02) * scale,
        vec3(0.34, 0.03, 0.22) * scale,
        None,
        Color::new(0.92, 0.78, 0.50, 1.0),
    );
    draw_cube(
        center + vec3(0.175, 0.155, -0.02) * scale,
        vec3(0.028, 0.028, 0.24) * scale,
        None,
        Color::new(0.85, 0.70, 0.44, 1.0),
    );

    // X marks the spot: five red pixels in a cross.
    let x_ink = Color::new(0.76, 0.18, 0.14, 1.0);
    let x_spot = vec3(-0.09, 0.172, -0.02);
    for (dx, dz) in [
        (0.0_f32, 0.0_f32),
        (0.024, 0.024),
        (-0.024, 0.024),
        (0.024, -0.024),
        (-0.024, -0.024),
    ] {
        draw_cube(
            center + (x_spot + vec3(dx, 0.0, dz)) * scale,
            vec3(0.022, 0.014, 0.022) * scale,
            None,
            x_ink,
        );
    }

    // Green island landmass and a dashed trail leading to the X.
    draw_cube(
        center + vec3(0.12, 0.17, 0.04) * scale,
        vec3(0.08, 0.025, 0.08) * scale,
        None,
        Color::new(0.38, 0.58, 0.36, 1.0),
    );
    let trail = Color::new(0.52, 0.34, 0.18, 1.0);
    for (x, z) in [
        (0.09_f32, -0.005_f32),
        (0.05, -0.030),
        (0.01, -0.045),
        (-0.04, -0.040),
    ] {
        draw_cube(
            center + vec3(x, 0.171, z) * scale,
            vec3(0.028, 0.014, 0.016) * scale,
            None,
            trail,
        );
    }

    // Compass rose in the corner with a gold north needle.
    let rose = vec3(-0.12, 0.172, 0.055);
    draw_cube(
        center + rose * scale,
        vec3(0.036, 0.012, 0.036) * scale,
        None,
        Color::new(0.94, 0.90, 0.80, 1.0),
    );
    draw_cube(
        center + rose * scale,
        vec3(0.055, 0.013, 0.010) * scale,
        None,
        trail,
    );
    draw_cube(
        center + rose * scale,
        vec3(0.010, 0.013, 0.055) * scale,
        None,
        trail,
    );
    draw_cube(
        center + (rose + vec3(0.0, 0.002, -0.036)) * scale,
        vec3(0.012, 0.014, 0.018) * scale,
        None,
        Color::new(0.92, 0.64, 0.18, 1.0),
    );

    // Gold coins spilled beside the map.
    let coin = Color::new(0.93, 0.76, 0.32, 1.0);
    for (x, z) in [(0.225_f32, -0.13_f32), (0.25, -0.08), (0.21, -0.045)] {
        draw_cube(
            center + vec3(x, 0.115, z) * scale,
            vec3(0.035, 0.012, 0.035) * scale,
            None,
            coin,
        );
    }
}
