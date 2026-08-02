//! Sessions staged for the screenshot harness (`TOYBOX_CAPTURE_SCENE`).
//!
//! These set up a situation a player would otherwise have to play their way
//! into. They are reachable only through the capture env vars, never during a
//! real run.

use crate::data::GameData;
use crate::state::{GamePhase, GameSession, RepairPartKind, WorldPoint};
use macroquad::prelude::*;

/// How much shop floor to sweep around the bench. At 4000 loose toys the
/// fixture under test is buried and the capture shows a wall of toys instead;
/// clearing the near field puts the bench back on camera while leaving the
/// stocked store visible behind it.
const BENCH_CLEARANCE: f32 = 7.0;

/// Two zones' lamp pools side by side across their shared boundary, floor
/// swept clean. The per-zone accent blend is subtle by design, and a frame full
/// of loose toys hides it entirely — this is the view that answers whether the
/// tint is doing enough work to be worth having.
pub fn lamp_contrast(data: &GameData) -> GameSession {
    let mut session = GameSession::new(data);

    // Pick the two adjacent zones whose accents differ most, so the comparison
    // is the fairest test the layout can offer rather than a flattering one.
    let zones = &data.layout.zones;
    let mut best = (0usize, 1usize, -1.0_f32);
    for (left_index, left) in zones.iter().enumerate() {
        for (right_index, right) in zones.iter().enumerate().skip(left_index + 1) {
            let touching = (left.x + left.w - right.x).abs() < 0.01
                || (right.x + right.w - left.x).abs() < 0.01;
            if !touching || (left.y - right.y).abs() > 0.01 {
                continue;
            }
            let separation: f32 = (0..3)
                .map(|channel| (left.accent[channel] - right.accent[channel]).abs())
                .sum();
            if separation > best.2 {
                best = (left_index, right_index, separation);
            }
        }
    }
    let right = &zones[best.1];
    // Stand directly under one of that zone's two pendant lamps and look down
    // at its floor pool. Anything further back and the pool is behind a
    // display fixture, which is how three earlier framings showed nothing.
    let stand = vec2(right.x + right.w * (0.5 - 0.26), right.y + right.h * 0.5);

    session
        .toys
        .retain(|toy| toy.position.to_vec2().distance(stand) > 6.0);
    let mut session = GameSession::from_save(session.to_save(&data.config.version), data);

    session.player.position = WorldPoint::from_vec2_for_capture(stand);
    session.player.yaw = -std::f32::consts::FRAC_PI_2;
    session.player.pitch = -GameSession::MAX_LOOK_PITCH;
    session
}

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

/// A shift with under a minute left. The HUD clock counts *down* in a timed
/// run and reddens as opening approaches, none of which a fresh shop shows.
pub fn closing_soon(data: &GameData) -> GameSession {
    let mut session = mid_run(data);
    session.player.elapsed_seconds = data.config.shift_seconds - 44.0;
    session
}

/// The score screen after the clock beat the player: two aisles restored, the
/// rest part-done, a handful of repairs and a few wrong shelves. A fresh
/// `TimeUp` session would score every row zero and prove nothing about layout.
pub fn shift_over(data: &GameData) -> GameSession {
    let mut session = GameSession::new(data);

    // Plush Corner and the Dragon Alcove finished; the rest partway.
    for display_index in 0..8 {
        stock_display(&mut session, data, display_index, 1.0);
    }
    for display_index in 8..14 {
        stock_display(&mut session, data, display_index, 0.55);
    }
    for display_index in 14..17 {
        stock_display(&mut session, data, display_index, 0.2);
    }

    session.player.repairs = 7;
    session.player.mistakes = 4;
    session.player.elapsed_seconds = data.config.shift_seconds;
    session.phase = GamePhase::TimeUp;
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
        session.pick_up_toy(toy_index, data);
        session.place_active_toy(display_index, slot_index, data);
    }
}

/// The tool shop with every tool unlocked and affordable, so the row layout
/// can be checked against the real list rather than a single entry.
pub fn tool_shop(data: &GameData) -> GameSession {
    let mut session = GameSession::new(data);
    // Credits come one per completed display; hand out enough that every row
    // renders its buyable state.
    for display_index in 0..data.displays.len().min(12) {
        stock_display(&mut session, data, display_index, 1.0);
    }
    session
}

/// The checkout, framed on the counter with the shopfront window behind it —
/// the view that shows the till clutter and the night sky through the glass.
pub fn checkout(data: &GameData) -> GameSession {
    let mut session = GameSession::new(data);
    let counter = data.layout.counters.first().expect("a checkout counter");
    // Stand between the counter and the shopfront window so one frame carries
    // both: the till clutter on the left, the night sky through the glass on
    // the right.
    // Between the counter and the shopfront so one frame carries both the till
    // clutter and the night sky through the glass.
    let stand = vec2(
        (counter.x + counter.w * 0.5 + data.layout.window.x) * 0.5,
        counter.y + counter.h + 1.6,
    );

    // Sweep the floor so the counter is not buried under 4000 loose toys.
    session
        .toys
        .retain(|toy| toy.position.to_vec2().distance(stand) > 6.0);
    let mut session = GameSession::from_save(session.to_save(&data.config.version), data);

    session.player.position = WorldPoint::from_vec2_for_capture(stand);
    // Face the shopfront wall, angled a little toward the window.
    session.player.yaw = -std::f32::consts::FRAC_PI_2 + 0.16;
    session.player.pitch = 0.06;
    session
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
        session.pick_up_toy(body_index, data);
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
