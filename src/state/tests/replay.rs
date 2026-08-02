//! Deterministic replays at shop scale.
//!
//! A scripted closer works the real `GameSession` API — the same `pick_up_toy`,
//! `interact` and `place_active_toy` the player drives — so scoring, mistakes,
//! credits, repairs and completion all come from the game rather than from a
//! model of it. Only travel is modelled: the closer teleports to each target
//! and pays the walk in seconds.
//!
//! Nothing here samples randomness, so two runs of the same scenario produce
//! byte-identical reports. That is what makes these useful for balance work:
//! change a number in `assets/data/*.json` and the difference in the report is
//! entirely attributable to that change.

use super::*;
use crate::state::WorldPoint;
use macroquad::prelude::Vec2;
use std::collections::HashSet;

/// A starting loadout to measure.
struct Scenario {
    name: &'static str,
    tools: &'static [&'static str],
}

const BEGINNER: Scenario = Scenario {
    name: "beginner",
    tools: &[],
};
const MID_UPGRADE: Scenario = Scenario {
    name: "mid-upgrade",
    tools: &["toy_scanner", "sorting_trolley"],
};
const FULLY_EQUIPPED: Scenario = Scenario {
    name: "fully equipped",
    tools: &[
        "toy_scanner",
        "sorting_trolley",
        "grippy_sneakers",
        "long_handled_grabber",
        "managers_nod",
    ],
};

/// Seconds of fiddling per interaction, on top of the walk. Not tuned against
/// a real player — it only has to be the same across scenarios for the
/// comparison between them to mean something.
const INTERACTION_SECONDS: f32 = 0.6;

#[derive(Debug, Clone, Copy, PartialEq)]
struct RunReport {
    actions: usize,
    shelved: usize,
    repaired: usize,
    mistakes: u32,
    deferred_parts: usize,
    displays_complete: usize,
    walked_metres: f32,
    minutes: f32,
}

impl RunReport {
    fn line(&self, name: &str) -> String {
        format!(
            "{name:>15}: {:>5} shelved  {:>3} repaired  {:>2} mistakes  \
             {:>3} deferred  {:>2} displays  {:>6.0}m walked  {:>5.1} min",
            self.shelved,
            self.repaired,
            self.mistakes,
            self.deferred_parts,
            self.displays_complete,
            self.walked_metres,
            self.minutes
        )
    }
}

/// Walk the closer to `target`, charging the clock for the distance.
fn walk_to(session: &mut GameSession, data: &GameData, target: WorldPoint, walked: &mut f32) {
    let from = session.player.position.to_vec2();
    let distance = from.distance(target.to_vec2());
    let speed = data.config.player_speed * session.speed_multiplier(data);
    session.player.position = target;
    session.player.elapsed_seconds += distance / speed.max(0.1) + INTERACTION_SECONDS;
    *walked += distance;
}

/// The closest toy still needing work, found through the real spatial grid so
/// the search cost does not dominate a 4000-action run.
fn nearest_loose_toy(session: &GameSession, from: Vec2, skip: &HashSet<usize>) -> Option<usize> {
    let mut radius = 2.0_f32;
    while radius <= 64.0 {
        let mut best: Option<(usize, f32)> = None;
        for index in session.spatial().indices_near(from, radius) {
            let toy = &session.toys[index];
            if skip.contains(&index)
                || toy.is_held
                || toy.placed_display_id.is_some()
                || toy.bench_slot_index.is_some()
                || toy.is_consumed_repair_part()
            {
                continue;
            }
            let distance = toy.position.to_vec2().distance(from);
            if best.is_none_or(|(_, best_distance)| distance < best_distance) {
                best = Some((index, distance));
            }
        }
        if let Some((index, _)) = best {
            return Some(index);
        }
        radius *= 2.0;
    }
    None
}

/// The nearest loose whole toy that belongs on `display`.
fn nearest_matching_toy(
    session: &GameSession,
    data: &GameData,
    display: &crate::data::DisplayDef,
    from: Vec2,
) -> Option<usize> {
    let mut radius = 2.0_f32;
    while radius <= 16.0 {
        let mut best: Option<(usize, f32)> = None;
        for index in session.spatial().indices_near(from, radius) {
            let toy = &session.toys[index];
            if toy.is_held
                || toy.placed_display_id.is_some()
                || toy.bench_slot_index.is_some()
                || toy.is_consumed_repair_part()
                || toy.is_repair_part()
                || !toy_matches_display(toy, display)
            {
                continue;
            }
            let distance = toy.position.to_vec2().distance(from);
            if best.is_none_or(|(_, best_distance)| distance < best_distance) {
                best = Some((index, distance));
            }
        }
        if let Some((index, _)) = best {
            return Some(index);
        }
        radius *= 2.0;
    }
    let _ = data;
    None
}

fn home_display_index(data: &GameData, toy: &ToyState) -> Option<usize> {
    data.displays
        .iter()
        .position(|display| toy_matches_display(toy, display))
}

/// Bench a carried repair part, repairing if its other half is already there.
/// Returns true when the closer walks away holding a repaired toy.
fn resolve_repair_part(session: &mut GameSession, data: &GameData, walked: &mut f32) -> bool {
    let bench = data.primary_bench();
    walk_to(
        session,
        data,
        WorldPoint {
            x: bench.x,
            y: bench.y,
        },
        walked,
    );
    session.interact(data);

    // Only reach for the repair when the bench actually holds a matching pair.
    // Interacting at a bench holding one lone part picks that part back up,
    // which is correct in game and would leave the closer stuck holding it.
    if session.bench_status(bench).stage == BenchStage::Ready
        && matches!(session.interact(data), InteractionResult::Repaired { .. })
    {
        return true;
    }
    if session.active_toy().is_some_and(|toy| toy.is_repair_part()) {
        session.drop_active(data);
    }
    false
}

fn run(scenario: &Scenario, data: &GameData, action_budget: usize) -> RunReport {
    let mut session = GameSession::new(data);
    for tool in scenario.tools {
        session.unlocked_upgrade_ids.push((*tool).to_owned());
    }

    // Where each display's next free slot is. Placement rejects a taken slot,
    // so the closer fills each shelf left to right like a person would.
    let mut next_slot = vec![0usize; data.displays.len()];
    let mut walked = 0.0;
    let (mut shelved, mut repaired, mut actions) = (0usize, 0usize, 0usize);
    // Parts the closer gave up on. Two mismatched halves fill a bench and
    // every later part then cycles pick-up, walk, refuse, drop forever, with
    // the closer parked at the bench so the dropped part is always the nearest
    // thing to it. Deferring is what a player does; without it the run stalls.
    let mut deferred: HashSet<usize> = HashSet::new();

    while actions < action_budget {
        let Some(first_index) =
            nearest_loose_toy(&session, session.player.position.to_vec2(), &deferred)
        else {
            break;
        };
        actions += 1;
        let target = session.toys[first_index].position;
        walk_to(&mut session, data, target, &mut walked);
        session.pick_up_toy(first_index, data);

        if session.toys[first_index].is_repair_part() {
            if resolve_repair_part(&mut session, data, &mut walked) {
                repaired += 1;
            } else {
                deferred.insert(first_index);
                continue;
            }
        }

        // Gather an armful bound for the same display before walking there.
        // A one-at-a-time routine would show no benefit from a bigger carry
        // limit at all, so this is what makes the trolley measurable.
        let Some(active) = session.active_toy().cloned() else {
            continue;
        };
        let Some(display_index) = home_display_index(data, &active) else {
            session.drop_active(data);
            continue;
        };
        let display = &data.displays[display_index];

        while session.player.carried_toy_ids.len() < session.carry_limit(data)
            && actions < action_budget
        {
            let from = session.player.position.to_vec2();
            let Some(extra) = nearest_matching_toy(&session, data, display, from) else {
                break;
            };
            actions += 1;
            let extra_position = session.toys[extra].position;
            walk_to(&mut session, data, extra_position, &mut walked);
            if !matches!(
                session.pick_up_toy(extra, data),
                InteractionResult::PickedUp { .. }
            ) {
                break;
            }
        }

        // Walk once, unload the armful.
        walk_to(
            &mut session,
            data,
            display_slot_position(display, next_slot[display_index], data.config.room_width),
            &mut walked,
        );
        while !session.player.carried_toy_ids.is_empty() {
            let slot = next_slot[display_index];
            if slot >= display.capacity {
                session.drop_active(data);
                continue;
            }
            match session.place_active_toy(display_index, slot, data) {
                InteractionResult::Placed { was_wrong, .. } => {
                    next_slot[display_index] += 1;
                    if !was_wrong {
                        shelved += 1;
                    }
                }
                _ => {
                    session.drop_active(data);
                }
            }
        }
    }

    RunReport {
        actions,
        shelved,
        repaired,
        mistakes: session.player.mistakes,
        deferred_parts: deferred.len(),
        displays_complete: session.completed_display_count(),
        walked_metres: walked,
        minutes: session.player.elapsed_seconds / 60.0,
    }
}

#[test]
fn a_replay_is_reproducible() {
    let data = GameData::load().unwrap();
    assert_eq!(run(&BEGINNER, &data, 300), run(&BEGINNER, &data, 300));
}

#[test]
fn the_closer_makes_real_progress_without_misshelving() {
    let data = GameData::load().unwrap();
    let report = run(&BEGINNER, &data, 400);

    assert_eq!(report.actions, 400);
    assert!(report.shelved > 0, "nothing reached a shelf");
    assert_eq!(
        report.mistakes, 0,
        "the closer only ever targets a toy's own display, so any mistake is \
         a bug in placement or matching"
    );
    assert!(report.walked_metres > 0.0);
}

/// The comparison the balance TODO needs: same shop, same script, different
/// loadout. Prints the table so a JSON retune can be re-measured.
#[test]
fn tools_pay_for_themselves_over_the_same_work() {
    let data = GameData::load().unwrap();
    let budget = 400;

    let reports: Vec<(&str, RunReport)> = [BEGINNER, MID_UPGRADE, FULLY_EQUIPPED]
        .iter()
        .map(|scenario| (scenario.name, run(scenario, &data, budget)))
        .collect();

    for (name, report) in &reports {
        println!("{}", report.line(name));
    }

    let beginner = reports[0].1;
    let equipped = reports[2].1;
    assert!(
        equipped.minutes < beginner.minutes,
        "a full toolset should finish the same {budget} actions faster: \
         {:.1} min equipped vs {:.1} min bare-handed",
        equipped.minutes,
        beginner.minutes
    );
}

/// The whole shop, start to finish. Slow by design — run it explicitly with
/// `cargo test --release full_shift -- --ignored --nocapture` when retuning.
#[test]
#[ignore = "full-shop replay: minutes to run, not part of the normal suite"]
fn a_full_shift_completes_the_shop() {
    let data = GameData::load().unwrap();
    for scenario in [BEGINNER, FULLY_EQUIPPED] {
        let report = run(&scenario, &data, data.config.toy_count * 2);
        println!("{}", report.line(scenario.name));
    }
}
