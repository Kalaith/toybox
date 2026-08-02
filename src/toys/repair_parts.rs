//! Per-category renderers for broken toy halves. Every part carries the
//! same gold break ring at the split line so parts read as "broken" at a
//! glance regardless of which toy they came from.

use super::primitives::{
    brighten, darken, draw_cube_with_edges, draw_eye_pair, draw_studded_block, draw_toy_sphere,
};
use crate::data::ToyCategory;
use crate::state::RepairPartKind;
use macroquad::prelude::*;

const BREAK_GOLD: Color = Color::new(0.98, 0.78, 0.34, 0.92);

pub(super) fn draw(
    category: ToyCategory,
    part: RepairPartKind,
    center: Vec3,
    color: Color,
    scale: f32,
) {
    match (category, part) {
        (ToyCategory::Plushies, RepairPartKind::Head) => plush_head(center, color, scale),
        (ToyCategory::Plushies, RepairPartKind::Body) => plush_body(center, color, scale),
        (ToyCategory::TinyDragons, RepairPartKind::Head) => dragon_head(center, color, scale),
        (ToyCategory::TinyDragons, RepairPartKind::Body) => dragon_body(center, color, scale),
        (ToyCategory::ActionFigures, RepairPartKind::Head) => robot_head(center, color, scale),
        (ToyCategory::ActionFigures, RepairPartKind::Body) => robot_body(center, color, scale),
        (ToyCategory::BoardGames, RepairPartKind::Head) => game_lid(center, color, scale),
        (ToyCategory::BoardGames, RepairPartKind::Body) => game_base(center, color, scale),
        (ToyCategory::BuildingBlocks, RepairPartKind::Head) => block_top(center, color, scale),
        (ToyCategory::BuildingBlocks, RepairPartKind::Body) => block_base(center, color, scale),
    }
}

/// Gold wire square marking the torn seam of a part.
fn break_ring(center: Vec3, y: f32, extent: f32, scale: f32) {
    draw_cube_wires(
        center + vec3(0.0, y, 0.0) * scale,
        vec3(extent, 0.02, extent) * scale,
        BREAK_GOLD,
    );
}

fn plush_head(center: Vec3, color: Color, scale: f32) {
    draw_toy_sphere(
        center + vec3(0.0, 0.20, 0.0) * scale,
        0.19 * scale,
        None,
        brighten(color, 0.06),
    );
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.14, 0.36, 0.0) * scale,
            0.07 * scale,
            None,
            color,
        );
    }
    draw_eye_pair(center, 0.22, -0.17, 0.07, scale);
    break_ring(center, 0.04, 0.30, scale);
}

fn plush_body(center: Vec3, color: Color, scale: f32) {
    draw_toy_sphere(
        center + vec3(0.0, 0.16, 0.0) * scale,
        0.21 * scale,
        None,
        color,
    );
    for side in [-1.0_f32, 1.0] {
        draw_toy_sphere(
            center + vec3(side * 0.20, 0.18, 0.0) * scale,
            0.08 * scale,
            None,
            darken(color, 0.06),
        );
        draw_toy_sphere(
            center + vec3(side * 0.10, 0.0, -0.04) * scale,
            0.09 * scale,
            None,
            darken(color, 0.06),
        );
    }
    break_ring(center, 0.36, 0.28, scale);
}

fn dragon_head(center: Vec3, color: Color, scale: f32) {
    let horn = Color::new(0.96, 0.88, 0.58, 1.0);
    draw_toy_sphere(
        center + vec3(0.0, 0.18, 0.0) * scale,
        0.16 * scale,
        None,
        brighten(color, 0.08),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.14, -0.15) * scale,
        0.08 * scale,
        None,
        brighten(color, 0.14),
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube_with_edges(
            center + vec3(side * 0.08, 0.34, 0.03) * scale,
            vec3(0.04, 0.10, 0.04) * scale,
            horn,
        );
    }
    draw_eye_pair(center, 0.21, -0.14, 0.06, scale);
    break_ring(center, 0.03, 0.26, scale);
}

fn dragon_body(center: Vec3, color: Color, scale: f32) {
    draw_toy_sphere(
        center + vec3(0.0, 0.14, 0.02) * scale,
        0.19 * scale,
        None,
        color,
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.21, 0.20, 0.06) * scale,
            vec3(0.05, 0.14, 0.18) * scale,
            None,
            darken(color, 0.10),
        );
    }
    draw_toy_sphere(
        center + vec3(0.0, 0.08, 0.24) * scale,
        0.08 * scale,
        None,
        brighten(color, 0.08),
    );
    break_ring(center, 0.32, 0.24, scale);
}

fn robot_head(center: Vec3, color: Color, scale: f32) {
    draw_cube(
        center + vec3(0.0, 0.18, 0.0) * scale,
        vec3(0.36, 0.28, 0.32) * scale,
        None,
        brighten(color, 0.08),
    );
    draw_eye_pair(center, 0.23, -0.18, 0.08, scale);
    draw_cube(
        center + vec3(0.0, 0.36, 0.0) * scale,
        vec3(0.055, 0.18, 0.055) * scale,
        None,
        darken(color, 0.18),
    );
    draw_toy_sphere(
        center + vec3(0.0, 0.48, 0.0) * scale,
        0.055 * scale,
        None,
        Color::new(0.94, 0.76, 0.28, 1.0),
    );
    break_ring(center, 0.02, 0.34, scale);
}

fn robot_body(center: Vec3, color: Color, scale: f32) {
    draw_cube(
        center + vec3(0.0, 0.12, 0.0) * scale,
        vec3(0.42, 0.36, 0.30) * scale,
        None,
        color,
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.32, 0.12, 0.0) * scale,
            vec3(0.12, 0.28, 0.12) * scale,
            None,
            darken(color, 0.12),
        );
    }
    break_ring(center, 0.34, 0.30, scale);
}

fn game_lid(center: Vec3, color: Color, scale: f32) {
    draw_cube_with_edges(
        center + vec3(0.0, 0.08, 0.0) * scale,
        vec3(0.42, 0.06, 0.40) * scale,
        brighten(color, 0.06),
    );
    draw_cube(
        center + vec3(0.0, 0.115, 0.0) * scale,
        vec3(0.28, 0.012, 0.16) * scale,
        None,
        Color::new(0.95, 0.93, 0.86, 1.0),
    );
    break_ring(center, 0.02, 0.40, scale);
}

fn game_base(center: Vec3, color: Color, scale: f32) {
    draw_cube_with_edges(
        center + vec3(0.0, 0.05, 0.0) * scale,
        vec3(0.42, 0.05, 0.40) * scale,
        darken(color, 0.06),
    );
    for side in [-1.0_f32, 1.0] {
        draw_cube(
            center + vec3(side * 0.19, 0.13, 0.0) * scale,
            vec3(0.04, 0.14, 0.40) * scale,
            None,
            color,
        );
        draw_cube(
            center + vec3(0.0, 0.13, side * 0.18) * scale,
            vec3(0.34, 0.14, 0.04) * scale,
            None,
            color,
        );
    }
    break_ring(center, 0.22, 0.40, scale);
}

fn block_top(center: Vec3, color: Color, scale: f32) {
    draw_studded_block(
        center + vec3(0.0, 0.12, 0.0) * scale,
        vec3(0.30, 0.16, 0.30) * scale,
        color,
    );
    break_ring(center, 0.02, 0.28, scale);
}

fn block_base(center: Vec3, color: Color, scale: f32) {
    draw_studded_block(
        center + vec3(0.0, 0.10, 0.0) * scale,
        vec3(0.38, 0.18, 0.34) * scale,
        color,
    );
    draw_studded_block(
        center + vec3(0.10, 0.26, -0.04) * scale,
        vec3(0.18, 0.12, 0.18) * scale,
        brighten(color, 0.08),
    );
    break_ring(center, 0.36, 0.24, scale);
}
