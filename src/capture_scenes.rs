//! Sessions staged for the screenshot harness (`TOYBOX_CAPTURE_SCENE`).
//!
//! These set up a situation a player would otherwise have to play their way
//! into. They are reachable only through the capture env vars, never during a
//! real run.

use crate::data::GameData;
use crate::state::{GameSession, RepairPartKind, WorldPoint};
use macroquad::prelude::*;

/// How much shop floor to sweep around the bench. At 4000 loose toys the
/// fixture under test is buried and the capture shows a wall of toys instead;
/// clearing the near field puts the bench back on camera while leaving the
/// stocked store visible behind it.
const BENCH_CLEARANCE: f32 = 7.0;

/// A run partway through: one aisle fully shelved, another half done, the rest
/// untouched. A fresh shop reads 0% everywhere, which shows nothing about the
/// per-zone HUD row or the minimap completion bars.
pub fn mid_run(data: &GameData) -> GameSession {
    let mut session = GameSession::new(data);

    // Displays 0-3 are the plush wall/bin/table; 4-7 the dragon alcove.
    for display_index in 0..4 {
        stock_display(&mut session, data, display_index, 1.0);
    }
    for display_index in 4..8 {
        stock_display(&mut session, data, display_index, 0.5);
    }

    // Stand in the plush corner so the HUD row reports a finished aisle.
    let zone = &data.layout.zones[0];
    session.player.position = WorldPoint {
        x: zone.x + zone.w * 0.5,
        y: zone.y + zone.h * 0.62,
    };
    session.player.yaw = -std::f32::consts::FRAC_PI_2;
    session.player.pitch = -0.08;
    session
}

/// Shelve `share` of one display's matching toys, straight into their slots.
fn stock_display(session: &mut GameSession, data: &GameData, display_index: usize, share: f32) {
    let display = &data.displays[display_index];
    let wanted = (display.capacity as f32 * share) as usize;
    let mut matching: Vec<(usize, String)> = session
        .toys
        .iter()
        .filter(|toy| {
            crate::state::toy_matches_display(toy, display)
                && toy.placed_display_id.is_none()
                && !toy.is_held
        })
        .map(|toy| (toy.slot_number, toy.id.clone()))
        .collect();
    matching.sort_by_key(|(slot_number, _)| *slot_number);

    for (slot_index, (_, toy_id)) in matching.into_iter().take(wanted).enumerate() {
        let Some(toy_index) = session.toys.iter().position(|toy| toy.id == toy_id) else {
            continue;
        };
        session.pick_up_toy(toy_index);
        session.place_active_toy(display_index, slot_index, data);
    }
}

/// A bench holding one half of a broken toy, framed from the front — the view
/// that shows the status beacon's `AwaitingMatch` state.
pub fn repair_bench(data: &GameData) -> GameSession {
    let bench = data.primary_bench();
    let mut session = GameSession::new(data);
    session.player.position = WorldPoint {
        x: bench.x,
        y: bench.y,
    };

    if let Some(body_index) = session
        .toys
        .iter()
        .position(|toy| toy.repair_part_kind() == Some(RepairPartKind::Body))
    {
        session.pick_up_toy(body_index);
        session.interact(data);
    }

    let bench_point = vec2(bench.x, bench.y);
    session.toys.retain(|toy| {
        toy.bench_slot_index.is_some()
            || toy.position.to_vec2().distance(bench_point) > BENCH_CLEARANCE
    });

    // The spatial grid is rebuilt on load, so round-trip the pruned session
    // rather than leaving the grid indexing toys that no longer exist.
    let mut session = GameSession::from_save(session.to_save(&data.config.version), data);
    // Inside the bench radius (1.7) so the AwaitingRepairMatch prompt fires,
    // and yawed a little off the waiting part so crosshair targeting does not
    // replace it with a pick-it-back-up prompt.
    session.player.position = WorldPoint {
        x: bench.x,
        y: bench.y - 1.45,
    };
    session.player.yaw = std::f32::consts::FRAC_PI_2 - 0.30;
    session.player.pitch = -0.10;
    session
}
