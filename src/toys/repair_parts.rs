//! Per-category renderers for broken toy halves. Every part carries the
//! same gold break ring at the split line so parts read as "broken" at a
//! glance regardless of which toy they came from.
//!
//! The ten renderers here are shared by all fifty identities; what tells a
//! broken Bear from a broken Octopus is the `PartAccent` each one is handed.
//! See `part_accents.rs` for why that is a table of numbers rather than a
//! hundred more models.

use super::library::ToyIdentity;
use super::part_accents::{accent_for, draw_crest, PartAccent};
use super::primitives::{
    brighten, darken, draw_cube_with_edges, draw_eye_pair, draw_studded_block, draw_toy_sphere,
};
use crate::data::ToyCategory;
use crate::state::RepairPartKind;
use macroquad::prelude::*;

const BREAK_GOLD: Color = Color::new(0.98, 0.78, 0.34, 0.92);

pub(super) fn draw(
    category: ToyCategory,
    identity: ToyIdentity,
    part: RepairPartKind,
    center: Vec3,
    color: Color,
    scale: f32,
) {
    let it = accent_for(identity);
    match (category, part) {
        (ToyCategory::Plushies, RepairPartKind::Head) => plush_head(it, center, color, scale),
        (ToyCategory::Plushies, RepairPartKind::Body) => plush_body(it, center, color, scale),
        (ToyCategory::TinyDragons, RepairPartKind::Head) => dragon_head(it, center, color, scale),
        (ToyCategory::TinyDragons, RepairPartKind::Body) => dragon_body(it, center, color, scale),
        (ToyCategory::ActionFigures, RepairPartKind::Head) => robot_head(it, center, color, scale),
        (ToyCategory::ActionFigures, RepairPartKind::Body) => robot_body(it, center, color, scale),
        (ToyCategory::BoardGames, RepairPartKind::Head) => game_lid(it, center, color, scale),
        (ToyCategory::BoardGames, RepairPartKind::Body) => game_base(it, center, color, scale),
        (ToyCategory::BuildingBlocks, RepairPartKind::Head) => block_top(it, center, color, scale),
        (ToyCategory::BuildingBlocks, RepairPartKind::Body) => block_base(it, center, color, scale),
    }
}

/// A snout, beak or sensor jutting from the face, sized by the accent. Skipped
/// entirely at `muzzle` 0.0 so flat-faced identities stay flat.
fn draw_muzzle(accent: PartAccent, center: Vec3, y: f32, color: Color, scale: f32) {
    if accent.muzzle <= 0.01 {
        return;
    }
    let reach = 0.10 + 0.11 * accent.muzzle;
    draw_toy_sphere(
        center + vec3(0.0, y, -reach) * scale,
        (0.045 + 0.030 * accent.muzzle) * scale,
        None,
        brighten(color, 0.12),
    );
}

/// Gold wire square marking the torn seam of a part.
fn break_ring(center: Vec3, y: f32, extent: f32, scale: f32) {
    draw_cube_wires(
        center + vec3(0.0, y, 0.0) * scale,
        vec3(extent, 0.02, extent) * scale,
        BREAK_GOLD,
    );
}

fn plush_head(accent: PartAccent, center: Vec3, color: Color, scale: f32) {
    draw_toy_sphere(
        center + vec3(0.0, 0.20, 0.0) * scale,
        0.19 * scale,
        None,
        brighten(color, 0.06),
    );
    draw_crest(accent, center, 0.33, 0.19, color, scale);
    draw_muzzle(accent, center, 0.18, color, scale);
    draw_eye_pair(center, 0.22, -0.17, 0.07 * accent.eye_spread, scale);
    break_ring(center, 0.04, 0.30, scale);
}

fn plush_body(accent: PartAccent, center: Vec3, color: Color, scale: f32) {
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
    draw_crest(accent, center, 0.34, 0.21, darken(color, 0.10), scale);
    break_ring(center, 0.36, 0.28, scale);
}

fn dragon_head(accent: PartAccent, center: Vec3, color: Color, scale: f32) {
    draw_toy_sphere(
        center + vec3(0.0, 0.18, 0.0) * scale,
        0.16 * scale,
        None,
        brighten(color, 0.08),
    );
    draw_muzzle(accent, center, 0.14, color, scale);
    draw_crest(accent, center, 0.30, 0.16, color, scale);
    draw_eye_pair(center, 0.21, -0.14, 0.06 * accent.eye_spread, scale);
    break_ring(center, 0.03, 0.26, scale);
}

fn dragon_body(accent: PartAccent, center: Vec3, color: Color, scale: f32) {
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
    draw_crest(accent, center, 0.30, 0.19, darken(color, 0.08), scale);
    break_ring(center, 0.32, 0.24, scale);
}

fn robot_head(accent: PartAccent, center: Vec3, color: Color, scale: f32) {
    draw_cube(
        center + vec3(0.0, 0.18, 0.0) * scale,
        vec3(0.36, 0.28, 0.32) * scale,
        None,
        brighten(color, 0.08),
    );
    draw_eye_pair(center, 0.23, -0.18, 0.08 * accent.eye_spread, scale);
    draw_muzzle(accent, center, 0.18, color, scale);
    // Brightened, not darkened: a robot crest is a thin aerial on a dark cube
    // in a dim shop, and the darker tone lost it against the head entirely.
    draw_crest(accent, center, 0.32, 0.18, brighten(color, 0.22), scale);
    break_ring(center, 0.02, 0.34, scale);
}

fn robot_body(accent: PartAccent, center: Vec3, color: Color, scale: f32) {
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
    draw_crest(accent, center, 0.30, 0.21, darken(color, 0.12), scale);
    break_ring(center, 0.34, 0.30, scale);
}

fn game_lid(accent: PartAccent, center: Vec3, color: Color, scale: f32) {
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
    draw_crest(accent, center, 0.12, 0.21, brighten(color, 0.10), scale);
    break_ring(center, 0.02, 0.40, scale);
}

fn game_base(accent: PartAccent, center: Vec3, color: Color, scale: f32) {
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
    draw_crest(accent, center, 0.20, 0.21, brighten(color, 0.08), scale);
    break_ring(center, 0.22, 0.40, scale);
}

fn block_top(accent: PartAccent, center: Vec3, color: Color, scale: f32) {
    draw_studded_block(
        center + vec3(0.0, 0.12, 0.0) * scale,
        vec3(0.30, 0.16, 0.30) * scale,
        color,
    );
    draw_crest(accent, center, 0.20, 0.15, brighten(color, 0.14), scale);
    break_ring(center, 0.02, 0.28, scale);
}

fn block_base(accent: PartAccent, center: Vec3, color: Color, scale: f32) {
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
    draw_crest(accent, center, 0.32, 0.19, brighten(color, 0.16), scale);
    break_ring(center, 0.36, 0.24, scale);
}
