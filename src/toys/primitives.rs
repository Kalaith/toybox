//! Shared 3D drawing primitives for the per-identity toy renderers.
//!
//! Everything here is stateless geometry: it knows nothing about which toy is
//! being drawn, only how to lay down the cubes and spheres the identity
//! modules compose into a toy.

use macroquad::prelude::*;

pub fn brighten(color: Color, amount: f32) -> Color {
    macroquad_toolkit::colors::lighten(color, amount)
}

pub fn darken(color: Color, amount: f32) -> Color {
    macroquad_toolkit::colors::darken(color, amount)
}

pub fn draw_cube_with_edges(center: Vec3, size: Vec3, color: Color) {
    draw_cube(center, size, None, color);
    draw_cube_wires(center, size, darken(color, 0.24));
    draw_cube(
        center + vec3(0.0, size.y * 0.51, -size.z * 0.08),
        vec3(size.x * 0.72, size.y * 0.045, size.z * 0.66),
        None,
        Color::new(1.0, 1.0, 0.90, 0.18),
    );
}

/// Drop-in `draw_sphere` replacement at 8x8 tessellation. Macroquad's
/// default is 16x16 and the mesh is regenerated per call per frame; toy
/// spheres are small on screen, so the low-poly version reads the same
/// at a quarter of the vertex cost.
pub fn draw_toy_sphere(center: Vec3, radius: f32, texture: Option<&Texture2D>, color: Color) {
    draw_sphere_ex(
        center,
        radius,
        texture,
        color,
        DrawSphereParams {
            rings: 8,
            slices: 8,
            ..Default::default()
        },
    );
}

pub fn draw_studded_block(center: Vec3, size: Vec3, color: Color) {
    draw_cube_with_edges(center, size, color);
    let stud_color = brighten(color, 0.10);
    let stud_radius = size.x.min(size.z) * 0.12;
    let y = center.y + size.y * 0.55;
    for x in [-0.24_f32, 0.24] {
        for z in [-0.22_f32, 0.22] {
            draw_toy_sphere(
                vec3(center.x + size.x * x, y, center.z + size.z * z),
                stud_radius,
                None,
                stud_color,
            );
        }
    }
}

pub fn draw_wheel(center: Vec3, radius: f32, width: f32, color: Color) {
    draw_toy_sphere(center, radius, None, color);
    draw_cube(
        center,
        vec3(radius * 1.32, width, radius * 1.32),
        None,
        darken(color, 0.06),
    );
    draw_toy_sphere(
        center,
        radius * 0.42,
        None,
        Color::new(0.82, 0.84, 0.80, 1.0),
    );
}

pub fn draw_face(center: Vec3, y: f32, z: f32, dx: f32, scale: f32) {
    draw_eye_pair(center, y, z, dx, scale);
    draw_face_mark(
        center + vec3(0.0, y - 0.04, z - 0.02) * scale,
        vec3(0.050, 0.034, 0.016) * scale,
    );
    draw_face_mark(
        center + vec3(-dx * 1.25, y - 0.055, z - 0.018) * scale,
        vec3(0.035, 0.018, 0.010) * scale,
    );
    draw_face_mark(
        center + vec3(dx * 1.25, y - 0.055, z - 0.018) * scale,
        vec3(0.035, 0.018, 0.010) * scale,
    );
}

pub fn draw_eye_pair(center: Vec3, y: f32, z: f32, dx: f32, scale: f32) {
    let size = vec3(0.052, 0.052, 0.016) * scale;
    draw_face_mark(center + vec3(-dx, y, z) * scale, size);
    draw_face_mark(center + vec3(dx, y, z) * scale, size);
    let sparkle = vec3(0.016, 0.016, 0.006) * scale;
    draw_cube(
        center + vec3(-dx - 0.010, y + 0.012, z - 0.010) * scale,
        sparkle,
        None,
        Color::new(0.92, 0.94, 0.90, 1.0),
    );
    draw_cube(
        center + vec3(dx - 0.010, y + 0.012, z - 0.010) * scale,
        sparkle,
        None,
        Color::new(0.92, 0.94, 0.90, 1.0),
    );
}

fn draw_face_mark(center: Vec3, size: Vec3) {
    draw_cube(center, size, None, Color::new(0.035, 0.030, 0.026, 1.0));
}

pub fn draw_dragon_base(center: Vec3, color: Color, scale: f32) {
    draw_toy_sphere(center, 0.24 * scale, None, color);
    draw_toy_sphere(
        center + vec3(0.0, -0.01, -0.15) * scale,
        0.15 * scale,
        None,
        Color::new(0.90, 0.76, 0.54, 1.0),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.18, -0.30) * scale,
        0.14 * scale,
        None,
        brighten(color, 0.08),
    );
    draw_cube(
        center + vec3(-0.27, 0.06, 0.03) * scale,
        vec3(0.10, 0.26, 0.38) * scale,
        None,
        darken(color, 0.10),
    );
    draw_cube(
        center + vec3(-0.31, 0.12, -0.03) * scale,
        vec3(0.055, 0.18, 0.26) * scale,
        None,
        brighten(color, 0.10),
    );
    draw_cube(
        center + vec3(0.27, 0.06, 0.03) * scale,
        vec3(0.10, 0.26, 0.38) * scale,
        None,
        darken(color, 0.10),
    );
    draw_cube(
        center + vec3(0.31, 0.12, -0.03) * scale,
        vec3(0.055, 0.18, 0.26) * scale,
        None,
        brighten(color, 0.10),
    );
    for x in [-0.12_f32, 0.12] {
        draw_cube(
            center + vec3(x, -0.18, -0.15) * scale,
            vec3(0.10, 0.08, 0.16) * scale,
            None,
            darken(color, 0.10),
        );
    }
    for index in 0..3 {
        draw_cube(
            center + vec3(0.0, 0.03 + index as f32 * 0.055, -0.23) * scale,
            vec3(0.11, 0.018, 0.030) * scale,
            None,
            Color::new(0.98, 0.86, 0.58, 1.0),
        );
    }
    draw_face(center, 0.20, -0.40, 0.06, scale);
}

pub fn draw_robot_core(center: Vec3, color: Color, scale: f32) {
    draw_cube_with_edges(
        center + vec3(0.0, 0.08, 0.0) * scale,
        vec3(0.34, 0.36, 0.30) * scale,
        color,
    );
    draw_cube_with_edges(
        center + vec3(0.0, 0.39, -0.01) * scale,
        vec3(0.28, 0.24, 0.26) * scale,
        brighten(color, 0.10),
    );
    draw_cube(
        center + vec3(0.0, 0.09, -0.17) * scale,
        vec3(0.18, 0.12, 0.025) * scale,
        None,
        Color::new(0.08, 0.12, 0.14, 1.0),
    );
    for x in [-0.07_f32, 0.07] {
        draw_toy_sphere(
            center + vec3(x, 0.11, -0.19) * scale,
            0.020 * scale,
            None,
            Color::new(0.56, 0.94, 0.88, 1.0),
        );
    }
    draw_eye_pair(center, 0.42, -0.16, 0.07, scale);
}

pub fn draw_robot_arms(center: Vec3, color: Color, scale: f32) {
    draw_cube_with_edges(
        center + vec3(-0.26, 0.08, 0.0) * scale,
        vec3(0.10, 0.30, 0.12) * scale,
        darken(color, 0.12),
    );
    draw_cube_with_edges(
        center + vec3(0.26, 0.08, 0.0) * scale,
        vec3(0.10, 0.30, 0.12) * scale,
        darken(color, 0.12),
    );
    for x in [-0.26_f32, 0.26] {
        draw_toy_sphere(
            center + vec3(x, -0.10, -0.01) * scale,
            0.055 * scale,
            None,
            Color::new(0.80, 0.84, 0.84, 1.0),
        );
    }
}

pub fn draw_game_box(center: Vec3, color: Color, scale: f32) {
    draw_cube_with_edges(center, vec3(0.56, 0.12, 0.40) * scale, color);
    draw_cube(
        center + vec3(0.0, 0.08, -0.01) * scale,
        vec3(0.46, 0.035, 0.30) * scale,
        None,
        brighten(color, 0.12),
    );
    draw_cube(
        center + vec3(-0.23, 0.105, -0.01) * scale,
        vec3(0.035, 0.020, 0.31) * scale,
        None,
        Color::new(0.95, 0.82, 0.38, 1.0),
    );
    draw_cube(
        center + vec3(0.06, 0.110, -0.14) * scale,
        vec3(0.22, 0.018, 0.030) * scale,
        None,
        Color::new(0.98, 0.94, 0.76, 1.0),
    );
}

pub fn shift_block_color(color: Color, index: usize) -> Color {
    match index % 3 {
        0 => color,
        1 => brighten(color, 0.10),
        _ => darken(color, 0.08),
    }
}
