//! Fixture footprints, movement blocking, and keep-off nudges.

use crate::data::GameData;
use macroquad::prelude::*;

const PLAYER_COLLISION_RADIUS: f32 = 0.45;

/// Footprints of everything solid on the floor: displays, aisle shelving,
/// and repair benches.
fn fixture_rects(data: &GameData) -> impl Iterator<Item = Rect> + '_ {
    let displays = data
        .displays
        .iter()
        .map(|display| Rect::new(display.x, display.y, display.w, display.h));
    let shelving = data
        .layout
        .shelving
        .iter()
        .chain(data.layout.counters.iter())
        .map(|shelf| Rect::new(shelf.x, shelf.y, shelf.w, shelf.h));
    let benches = data.layout.benches.iter().map(|bench| {
        Rect::new(
            bench.x - bench.w * 0.5,
            bench.y - bench.h * 0.5,
            bench.w,
            bench.h,
        )
    });
    displays.chain(shelving).chain(benches)
}

pub(super) fn position_blocked(position: Vec2, data: &GameData) -> bool {
    fixture_rects(data).any(|rect| {
        position.x > rect.x - PLAYER_COLLISION_RADIUS
            && position.x < rect.right() + PLAYER_COLLISION_RADIUS
            && position.y > rect.y - PLAYER_COLLISION_RADIUS
            && position.y < rect.bottom() + PLAYER_COLLISION_RADIUS
    })
}

pub(super) fn keep_off_fixtures(mut position: Vec2, data: &GameData) -> Vec2 {
    // Nudge targets must stay inside the playable band, or the caller's
    // room clamp would shove a wall-side nudge straight back into the
    // fixture it escaped.
    let edge = 0.8;
    let max_x = data.config.room_width - edge;
    let max_y = data.config.room_height - edge;
    let in_band = |nudged: &Vec2| {
        nudged.x >= edge && nudged.x <= max_x && nudged.y >= edge && nudged.y <= max_y
    };

    // Two passes: escaping one fixture can land inside a neighbor.
    for _ in 0..2 {
        for rect in fixture_rects(data) {
            let margin = 0.18;
            let left = rect.x - margin;
            let right = rect.right() + margin;
            let top = rect.y - margin;
            let bottom = rect.bottom() + margin;
            if position.x < left || position.x > right || position.y < top || position.y > bottom {
                continue;
            }

            let distances = [
                (position.x - left, vec2(left - 0.26, position.y)),
                (right - position.x, vec2(right + 0.26, position.y)),
                (position.y - top, vec2(position.x, top - 0.26)),
                (bottom - position.y, vec2(position.x, bottom + 0.26)),
            ];
            position = distances
                .iter()
                .filter(|(_, nudged)| in_band(nudged))
                .min_by(|(left_distance, _), (right_distance, _)| {
                    left_distance.total_cmp(right_distance)
                })
                .map(|(_, nudged)| *nudged)
                .unwrap_or(position);
        }
    }

    position
}
