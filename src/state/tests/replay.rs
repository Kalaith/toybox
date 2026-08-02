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
use macroquad::prelude::{vec2, Vec2};
use std::collections::HashSet;

/// How the closer decides what to do next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Strategy {
    /// Shelve whatever is nearest, benching repair parts only when one happens
    /// to be underfoot. This measures the sorting loop.
    NearestFirst,
    /// Hunt broken pairs on purpose: take a half, go fetch its other half from
    /// wherever in the shop it landed, rejoin them, then shelve the result.
    /// This measures the repair loop, which the scatter is designed around.
    Restorer,
    /// Sort like `NearestFirst`, but spend credits the moment they arrive.
    ///
    /// This is the only loadout a real timed run ever has: tools do not carry
    /// between shifts, so nobody starts equipped and nobody stays bare-handed.
    /// The other two scenarios bracket the run; this one is the run.
    Earner,
}

/// A starting loadout and a way of playing, to measure together.
struct Scenario {
    name: &'static str,
    tools: &'static [&'static str],
    strategy: Strategy,
}

const BEGINNER: Scenario = Scenario {
    name: "beginner",
    tools: &[],
    strategy: Strategy::NearestFirst,
};
const MID_UPGRADE: Scenario = Scenario {
    name: "mid-upgrade",
    tools: &["toy_scanner", "sorting_trolley"],
    strategy: Strategy::NearestFirst,
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
    strategy: Strategy::NearestFirst,
};
const EARNER: Scenario = Scenario {
    name: "earns tools",
    tools: &[],
    strategy: Strategy::Earner,
};
const RESTORER: Scenario = Scenario {
    name: "restorer",
    tools: &["toy_scanner"],
    strategy: Strategy::Restorer,
};
const RESTORER_EQUIPPED: Scenario = Scenario {
    name: "restorer+tools",
    tools: &[
        "toy_scanner",
        "sorting_trolley",
        "grippy_sneakers",
        "long_handled_grabber",
    ],
    strategy: Strategy::Restorer,
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
    /// Times the crosshair delivered nothing at all — the closer walked to a
    /// toy, aimed at it, and `E` did not pick anything up.
    whiffs: usize,
    /// Times the crosshair delivered a *different* toy than the one aimed at,
    /// because a neighbour in the pile sat nearer the centre of the view. The
    /// closer still walks away holding something, so this is pile texture, not
    /// lost work.
    grabbed_neighbour: usize,
    deferred_parts: usize,
    displays_complete: usize,
    /// Toys still on the floor when the run stopped. Together with `actions`
    /// this says whether a short run ran out of budget or stalled: a run that
    /// spent its whole budget and left most of the shop loose is doing
    /// something other than shelving.
    still_loose: usize,
    walked_metres: f32,
    minutes: f32,
}

impl RunReport {
    fn line(&self, name: &str) -> String {
        format!(
            "{name:>15}: {:>5} actions  {:>5} shelved  {:>5} loose  {:>3} repaired  \
             {:>2} mistakes  {:>4} deferred  {:>4} whiffed  {:>4} neighbour  \
             {:>2} displays  {:>6.0}m walked  {:>6.1} min",
            self.actions,
            self.shelved,
            self.still_loose,
            self.repaired,
            self.mistakes,
            self.deferred_parts,
            self.whiffs,
            self.grabbed_neighbour,
            self.displays_complete,
            self.walked_metres,
            self.minutes
        )
    }
}

/// Walk the closer to arm's length of `target` and turn to face it, charging
/// the clock for the distance. Stopping short matters: standing exactly on a
/// toy puts it at zero range, and the game's crosshair targeting rejects
/// anything that is not actually in front of the player.
/// Mirrors `PLAYER_EYE_HEIGHT` in `state/interactions.rs`, which is private.
const EYE_HEIGHT: f32 = 1.08;
/// Roughly where a loose toy's aim centre sits above the floor.
const TOY_AIM_HEIGHT: f32 = 0.22;
/// How far short of a target the closer stops.
const STANDOFF: f32 = 0.55;

fn walk_to(session: &mut GameSession, data: &GameData, target: WorldPoint, walked: &mut f32) {
    let from = session.player.position.to_vec2();
    let to_target = target.to_vec2() - from;
    let distance = to_target.length();
    let arrival = if distance > STANDOFF {
        target.to_vec2() - to_target.normalize() * STANDOFF
    } else {
        from
    };

    let speed = data.config.player_speed * session.speed_multiplier(data);
    session.player.position = WorldPoint::from_vec2_for_replay(arrival);
    if to_target.length_squared() > f32::EPSILON {
        session.player.yaw = to_target.y.atan2(to_target.x);
    }
    session.player.elapsed_seconds += distance / speed.max(0.1) + INTERACTION_SECONDS;
    *walked += distance;
}

/// Pick a toy up the way a player does: stand in front of it, look at it, and
/// press E — but only once the game agrees that E will pick something up.
///
/// `interact` dispatches on context, so a script cannot simply press E and
/// assume. Near a display while holding something it shelves instead, which an
/// earlier version of this harness discovered by inventing mis-shelvings that
/// were its own fault. `interaction_preview` is the game's own answer to "what
/// would E do here", so checking it first is what makes driving the input
/// layer faithful rather than reckless.
///
/// Returns the toy actually picked up, which is not always the one aimed at:
/// in a pile the crosshair takes whatever is nearest the centre of the view.
/// Callers must work with what they got rather than forcing what they wanted —
/// forcing it is how the first attempt at this ended up holding one toy and
/// shelving it on another's display.
fn aim_and_pick_up(
    session: &mut GameSession,
    data: &GameData,
    intended: usize,
    whiffs: &mut usize,
    neighbours: &mut usize,
) -> Option<usize> {
    // Look down at the floor, not out across it. A fixed shallow pitch aims
    // over the top of a toy standing at arm's length and misses almost every
    // time — the first measured pass reported an 89% miss rate for exactly
    // this reason, and read it as the pile being crowded.
    let ground = session
        .player
        .position
        .to_vec2()
        .distance(session.toys[intended].position.to_vec2())
        .max(STANDOFF * 0.5);
    session.player.pitch = ((TOY_AIM_HEIGHT - EYE_HEIGHT) / ground)
        .atan()
        .clamp(-GameSession::MAX_LOOK_PITCH, GameSession::MAX_LOOK_PITCH);

    if !matches!(
        session.interaction_preview(data),
        InteractionPreview::Pickup { .. }
    ) {
        *whiffs += 1;
        return None;
    }

    if !matches!(session.interact(data), InteractionResult::PickedUp { .. }) {
        *whiffs += 1;
        return None;
    }

    let held = session.active_toy()?.id.clone();
    let got = session.toys.iter().position(|toy| toy.id == held)?;
    if got != intended {
        *neighbours += 1;
    }
    Some(got)
}

/// Take a *named* toy, bypassing the crosshair.
///
/// The restoration errand needs one specific half, and at 4000 loose toys the
/// crosshair cannot reliably deliver a named toy — measured, the closer failed
/// to dig out its intended part on essentially every attempt even given six
/// tries each, and completed zero repairs. Sorting does not have this problem
/// because any toy in the pile is a fine toy to shelve, which is why that
/// strategy runs on real input above.
///
/// So this measures the errand's *travel and rejoin* cost with the digging
/// abstracted away. The digging cost is real and is not in these numbers; see
/// the aim-miss column on the sorting rows for its scale.
fn take_directly(session: &mut GameSession, data: &GameData, toy_index: usize) -> bool {
    matches!(
        session.pick_up_toy(toy_index, data),
        InteractionResult::PickedUp { .. }
    )
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

/// The nearest loose repair part, wherever in the shop it is. The restorer
/// searches the whole room rather than a local radius: the errand is the point.
fn nearest_loose_part(session: &GameSession, from: Vec2, skip: &HashSet<usize>) -> Option<usize> {
    session
        .toys
        .iter()
        .enumerate()
        .filter(|(index, toy)| {
            !skip.contains(index)
                && toy.is_repair_part()
                && !toy.is_held
                && toy.bench_slot_index.is_none()
        })
        .min_by(|(_, left), (_, right)| {
            let left_distance = left.position.to_vec2().distance_squared(from);
            let right_distance = right.position.to_vec2().distance_squared(from);
            left_distance.total_cmp(&right_distance)
        })
        .map(|(index, _)| index)
}

/// The other half of a broken toy: the part sharing its `repair_id`.
fn counterpart_index(session: &GameSession, toy_index: usize) -> Option<usize> {
    let RepairState::BrokenPart { repair_id, .. } = &session.toys[toy_index].repair_state else {
        return None;
    };
    let own_id = &session.toys[toy_index].id;
    session.toys.iter().position(|toy| {
        &toy.id != own_id
            && matches!(
                &toy.repair_state,
                RepairState::BrokenPart { repair_id: other, .. } if other == repair_id
            )
    })
}

/// Where a toy belongs: the nearest display of its category and theme that
/// still has a free slot.
///
/// Every category owns four displays, so "the first one that matches" is not a
/// shelf-choosing rule, it is a way to ignore three quarters of the shop. A
/// closer using it fills 1000 of the 4000 slots, and then every remaining toy
/// it picks up is carried to a shelf that has been full for hours and dropped
/// again — 9700 wasted actions in an 8000-action shift, which is what made a
/// full shop look unfinishable. Room first, then distance, because a player
/// who walks to a full shelf has to walk somewhere else anyway.
fn home_display_index(
    data: &GameData,
    toy: &ToyState,
    next_slot: &[usize],
    from: Vec2,
) -> Option<usize> {
    data.displays
        .iter()
        .enumerate()
        .filter(|(index, display)| {
            toy_matches_display(toy, display) && next_slot[*index] < display.capacity
        })
        .min_by(|(_, left), (_, right)| {
            display_centre(left)
                .distance_squared(from)
                .total_cmp(&display_centre(right).distance_squared(from))
        })
        .map(|(index, _)| index)
}

fn display_centre(display: &crate::data::DisplayDef) -> Vec2 {
    vec2(display.x + display.w * 0.5, display.y + display.h * 0.5)
}

/// Bench a carried repair part, repairing if its other half is already there.
/// Returns true when the closer walks away holding a repaired toy.
fn resolve_repair_part(session: &mut GameSession, data: &GameData, walked: &mut f32) -> bool {
    // If the other half is already waiting on a bench, go to *that* bench.
    // This is the route the Toy Scanner exists to give the player; without it
    // a closer walks to the nearest bench, is refused because that bench holds
    // someone else's half, and the pair never meets.
    let counterpart = session.carried_counterpart();
    let target = match counterpart {
        Some(location) if location.on_bench => location.position,
        _ => {
            let bench = data.primary_bench();
            WorldPoint {
                x: bench.x,
                y: bench.y,
            }
        }
    };
    walk_to(session, data, target, walked);
    session.interact(data);

    // Only reach for the repair when the bench actually holds a matching pair.
    // Interacting at a bench holding one lone part picks that part back up,
    // which is correct in game and would leave the closer stuck holding it.
    if session.bench_status(data.primary_bench()).stage == BenchStage::Ready
        && matches!(session.interact(data), InteractionResult::Repaired { .. })
    {
        return true;
    }
    if session.active_toy().is_some_and(|toy| toy.is_repair_part()) {
        session.drop_active(data);
    }
    false
}

/// One full restoration errand: take a half, cross the shop for its other half,
/// rejoin them at a bench. Returns the toy index of the repaired toy, now in
/// the closer's hands and ready to shelve.
fn restore_one_pair(
    session: &mut GameSession,
    data: &GameData,
    walked: &mut f32,
    actions: &mut usize,
    deferred: &mut HashSet<usize>,
) -> Option<usize> {
    let from = session.player.position.to_vec2();
    let first = nearest_loose_part(session, from, deferred)?;
    let mate = counterpart_index(session, first)?;
    if session.toys[mate].bench_slot_index.is_some() || session.toys[mate].is_held {
        deferred.insert(first);
        return None;
    }

    // Half one: fetch it and park it on a bench.
    *actions += 1;
    let first_position = session.toys[first].position;
    walk_to(session, data, first_position, walked);
    if !take_directly(session, data, first) {
        deferred.insert(first);
        return None;
    }
    if resolve_repair_part(session, data, walked) {
        return Some(first);
    }
    if session.toys[first].bench_slot_index.is_none() {
        // The bench would not take it — nothing more to try this errand.
        deferred.insert(first);
        return None;
    }

    // Half two: cross the shop for it, then carry it to the bench holding the
    // first half. `resolve_repair_part` routes there via the scanner.
    *actions += 1;
    let mate_position = session.toys[mate].position;
    walk_to(session, data, mate_position, walked);
    if !take_directly(session, data, mate) {
        deferred.insert(mate);
        return None;
    }
    if resolve_repair_part(session, data, walked) {
        // The survivor of a repair is the body, whichever index that is.
        return session
            .active_toy()
            .and_then(|toy| session.toys.iter().position(|other| other.id == toy.id));
    }
    deferred.insert(first);
    deferred.insert(mate);
    None
}

/// Spend credits as soon as they cover the next unlocked tool, cheapest first.
///
/// A player opens the tool screen when the HUD tells them something is
/// affordable, so buying eagerly is the faithful behaviour. Cheapest first
/// matters: the Toy Scanner at one credit arrives on the first completed
/// display, and holding out for a three-credit tool would leave the closer
/// unequipped through the part of the run where help compounds most.
fn buy_what_it_can_afford(session: &mut GameSession, data: &GameData) {
    loop {
        let mut affordable: Vec<&crate::data::UpgradeDef> = data
            .upgrades
            .iter()
            .filter(|upgrade| {
                !session.has_upgrade(&upgrade.id)
                    && session.completed_display_count() >= upgrade.unlock_completed_displays
                    && session.available_tool_credits(data) >= upgrade.cost
            })
            .collect();
        affordable.sort_by_key(|upgrade| upgrade.cost);

        let Some(upgrade) = affordable.first() else {
            return;
        };
        if !matches!(
            session.purchase_tool(data, &upgrade.id),
            ToolPurchaseResult::Purchased { .. }
        ) {
            return;
        }
    }
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
    let (mut whiffs, mut neighbours) = (0usize, 0usize);
    // Parts the closer gave up on. Two mismatched halves fill a bench and
    // every later part then cycles pick-up, walk, refuse, drop forever, with
    // the closer parked at the bench so the dropped part is always the nearest
    // thing to it. Deferring is what a player does; without it the run stalls.
    let mut deferred: HashSet<usize> = HashSet::new();

    while actions < action_budget {
        if scenario.strategy == Strategy::Earner {
            buy_what_it_can_afford(&mut session, data);
        }
        if scenario.strategy == Strategy::Restorer {
            // Hunt a pair. On success the closer is holding a repaired toy and
            // falls through to shelving it like any other.
            let before = actions;
            match restore_one_pair(&mut session, data, &mut walked, &mut actions, &mut deferred) {
                Some(_) => repaired += 1,
                None => {
                    if actions == before {
                        break; // no pairs left to chase
                    }
                    continue;
                }
            }
        } else {
            let Some(first_index) =
                nearest_loose_toy(&session, session.player.position.to_vec2(), &deferred)
            else {
                break;
            };
            actions += 1;
            let target = session.toys[first_index].position;
            walk_to(&mut session, data, target, &mut walked);
            let Some(held) = aim_and_pick_up(
                &mut session,
                data,
                first_index,
                &mut whiffs,
                &mut neighbours,
            ) else {
                deferred.insert(first_index);
                continue;
            };

            if session.toys[held].is_repair_part() {
                if resolve_repair_part(&mut session, data, &mut walked) {
                    repaired += 1;
                } else {
                    deferred.insert(held);
                    continue;
                }
            }
        }

        // Gather an armful bound for the same display before walking there.
        // A one-at-a-time routine would show no benefit from a bigger carry
        // limit at all, so this is what makes the trolley measurable.
        let Some(active) = session.active_toy().cloned() else {
            continue;
        };
        let Some(display_index) =
            home_display_index(data, &active, &next_slot, session.player.position.to_vec2())
        else {
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
            if aim_and_pick_up(&mut session, data, extra, &mut whiffs, &mut neighbours).is_none() {
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
            // An armful gathered for one display can still contain a stray: the
            // crosshair hands over whatever sat nearest the centre of the view,
            // not always the toy aimed at. A player checks what is in hand
            // before shelving it. Forcing it here would manufacture mistakes
            // and blind the run's mistake count to real placement bugs.
            let belongs = session
                .active_toy()
                .is_some_and(|toy| toy_matches_display(toy, display));
            if !belongs || slot >= display.capacity {
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
        whiffs,
        grabbed_neighbour: neighbours,
        deferred_parts: deferred.len(),
        displays_complete: session.completed_display_count(),
        still_loose: session
            .toys
            .iter()
            .filter(|toy| {
                !toy.is_held && toy.placed_display_id.is_none() && !toy.is_consumed_repair_part()
            })
            .count(),
        walked_metres: walked,
        minutes: session.player.elapsed_seconds / 60.0,
    }
}

#[test]
fn a_replay_is_reproducible() {
    let data = GameData::load().unwrap();
    assert_eq!(run(&BEGINNER, &data, 300), run(&BEGINNER, &data, 300));
}

/// A bare-handed closer must be able to empty the floor of whole toys, and to
/// do it without spinning. Both halves matter: a closer that only ever walks to
/// the *first* display of each category fills a quarter of the shop and then
/// carries every remaining toy to a shelf that has been full for hours, and the
/// only symptom is a run that burns its whole budget with the floor still
/// covered. Asserting on the budget is what hid that for so long.
#[test]
fn the_closer_clears_the_floor_without_misshelving() {
    let data = GameData::load().unwrap();
    let budget = data.config.toy_count * 4;
    let report = run(&BEGINNER, &data, budget);

    assert!(report.actions < budget, "the closer never ran out of work");
    assert_eq!(
        report.mistakes, 0,
        "the closer only ever targets a toy's own display, so any mistake is \
         a bug in placement or matching"
    );
    assert!(report.walked_metres > 0.0);

    // What is left on the floor should be the repair parts this strategy
    // deliberately defers, not whole toys it could not find a home for.
    // Saturating because the deferred set can outnumber what is actually loose:
    // a toy the crosshair handed over as a neighbour gets shelved even though
    // its index was written off earlier.
    let whole_left = report.still_loose.saturating_sub(report.deferred_parts);
    assert!(
        whole_left * 20 < data.config.toy_count,
        "{whole_left} whole toys never reached a shelf out of {}: the closer is \
         running out of somewhere to put them",
        data.config.toy_count
    );
}

/// Nothing the renderer reads may reach the score. These five values decide
/// which toys get drawn, at what detail, and how much of the view is culled —
/// change them to their most aggressive settings and the run must come out
/// byte-identical. If one ever leaks into targeting or placement, a player who
/// nudged their view distance would quietly be playing a different game.
#[test]
fn render_settings_cannot_move_the_score() {
    let baseline = GameData::load().unwrap();
    let mut altered = baseline.clone();
    // Deliberately below `interaction_radius` (1.35). A render value larger
    // than the reach cannot change a targeting decision even if gameplay does
    // read it, so a gentler setting here makes the whole test vacuous.
    altered.config.toy_render_distance = 0.4;
    altered.config.toy_lod_distance = 0.25;
    altered.config.toy_pose_distance = 0.25;
    altered.config.toy_view_cull_min_dot = 0.98;
    altered.config.toy_always_draw_radius = 0.0;

    // Crosshair targeting, which the scripted runs below deliberately bypass.
    // `interaction_preview` is the public read of `targeted_loose_toy_index`,
    // the one place a render distance could plausibly be mistaken for a
    // gameplay reach.
    let mut session = GameSession::new(&baseline);
    let toy = session
        .toys
        .iter()
        .position(|toy| !toy.is_held && toy.placed_display_id.is_none())
        .unwrap();
    let toy_position = session.toys[toy].position.to_vec2();
    session.player.position = WorldPoint::from_vec2_for_replay(toy_position - vec2(0.5, 0.0));
    session.player.yaw = 0.0;
    session.player.pitch = -0.42;
    assert!(
        matches!(
            session.interaction_preview(&baseline),
            InteractionPreview::Pickup { .. }
        ),
        "the targeting probe must actually be looking at a toy, or it proves nothing"
    );
    assert!(
        matches!(
            session.interaction_preview(&altered),
            InteractionPreview::Pickup { .. }
        ),
        "render distance changed what the crosshair can pick up"
    );

    assert_eq!(
        run(&BEGINNER, &baseline, 250),
        run(&BEGINNER, &altered, 250),
        "render distance / LOD / culling changed the sorting run"
    );
    assert_eq!(
        run(&RESTORER, &baseline, 150),
        run(&RESTORER, &altered, 150),
        "render distance / LOD / culling changed the restoration run"
    );
}

/// The repair loop is the half of the game the scatter is built around: cross
/// the shop for a missing head, rejoin it, shelve the whole toy. A nearest-first
/// closer never runs that errand, so it needs measuring on its own terms.
#[test]
fn the_restoration_errand_is_winnable() {
    let data = GameData::load().unwrap();

    for scenario in [RESTORER, RESTORER_EQUIPPED] {
        let report = run(&scenario, &data, 200);
        println!("{}", report.line(scenario.name));
        assert!(
            report.repaired > 0,
            "{} completed no repairs: the hunt-and-rejoin loop is unplayable,              not merely slow",
            scenario.name
        );
        assert!(
            report.shelved >= report.repaired,
            "every repaired toy should reach its shelf"
        );
    }
}

/// Can a shift actually be finished before the doors open?
///
/// `shift_seconds` was set from the bare-handed replay, but nobody plays a
/// bare-handed run: tools do not carry between shifts, so every timed run
/// starts with nothing and earns its way up. This measures that run, and the
/// two bracketing loadouts alongside it, against the real deadline.
#[test]
fn the_deadline_is_reachable_by_a_closer_who_buys_tools() {
    let data = GameData::load().unwrap();
    let budget = data.config.toy_count * 4;
    let limit = data.config.shift_seconds / 60.0;

    let reports: Vec<(&str, RunReport)> = [BEGINNER, EARNER, FULLY_EQUIPPED]
        .iter()
        .map(|scenario| (scenario.name, run(scenario, &data, budget)))
        .collect();

    for (name, report) in &reports {
        println!(
            "{}   [deadline {limit:.0} min, {:+.1}]",
            report.line(name),
            limit - report.minutes
        );
    }

    let earner = reports[1].1;
    assert!(
        earner.minutes < limit,
        "a closer that buys tools as it earns them cannot clear the shop before \
         opening: {:.1} min against a {limit:.0} min shift. Either shift_seconds \
         is too tight or the tools are too weak.",
        earner.minutes
    );
    assert!(
        earner.minutes < reports[0].1.minutes,
        "buying tools did not beat staying bare-handed"
    );
}

/// What a wrong shelf should cost, expressed in the only unit the player
/// experiences: another toy's worth of work.
///
/// A mis-shelving already costs real time — the toy sits in a slot without
/// counting, so it blocks its display and has to be collected and re-shelved.
/// `mistake_penalty_seconds` is the surcharge on top, and the rule it encodes
/// is "one more toy". That is a number nobody can eyeball, because seconds per
/// toy falls out of shop size, walking speed and tools all at once; this ties
/// the two together so a retune of any of them shows up here instead of
/// silently making the penalty trivial or savage.
#[test]
fn a_wrong_shelf_costs_about_one_toys_worth_of_time() {
    let data = GameData::load().unwrap();
    let report = run(&BEGINNER, &data, data.config.toy_count * 4);

    let seconds_per_toy = report.minutes * 60.0 / report.shelved as f32;
    let penalty = data.config.mistake_penalty_seconds;
    let ratio = penalty / seconds_per_toy;

    println!(
        "{:.1}s per toy, {penalty:.1}s penalty, ratio {ratio:.2}",
        seconds_per_toy
    );
    assert!(
        (0.5..=2.0).contains(&ratio),
        "a wrong shelf costs {ratio:.2} toys' worth of time ({penalty:.1}s against \
         {seconds_per_toy:.1}s per toy). Below 0.5 the penalty is not felt; above \
         2.0 a single slip outweighs the work it interrupted."
    );
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

    for (name, report) in &reports {
        assert_eq!(
            report.mistakes, 0,
            "{name} mis-shelved something: the closer only ever shelves a toy on \
             its own display, so a mistake here is a bug in placement or matching"
        );
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
        let report = run(&scenario, &data, data.config.toy_count * 4);
        println!("{}", report.line(scenario.name));
    }
}

/// What a shift costs at each shop size, for deciding `toy_count`.
///
/// Every display holds the same number, so one knob moves the whole shop and
/// keeps the capacity-equals-`toy_count` invariant intact. Run it with
/// `cargo test --release shop_scale -- --ignored --nocapture`.
///
/// Note that `displays` stays near zero at every size: a display cannot
/// complete until its broken toys are rejoined, and this closer defers the
/// repair errand rather than running it. Read the shelved/minutes columns for
/// length and `the_restoration_errand_is_winnable` for the other half.
#[test]
#[ignore = "shop-scaling sweep: for retuning toy_count, not part of the normal suite"]
fn shop_scale_sets_the_length_of_a_shift() {
    let base = GameData::load().unwrap();

    for per_display in [8usize, 12, 20, 40, 100, 200] {
        let mut data = base.clone();
        for display in &mut data.displays {
            display.capacity = per_display;
        }
        data.config.toy_count = per_display * data.displays.len();

        for scenario in [BEGINNER, FULLY_EQUIPPED] {
            let report = run(&scenario, &data, data.config.toy_count * 4);
            println!(
                "cap {per_display:>3} / {:>4} toys  {}",
                data.config.toy_count,
                report.line(scenario.name)
            );
        }
    }
}
