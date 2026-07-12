use super::*;
use crate::toys::{toy_color, toy_profile};
use std::collections::HashSet;

#[test]
fn new_session_generates_requested_toys() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let final_toy_count = session
        .toys
        .iter()
        .filter(|toy| {
            !matches!(
                toy.repair_state,
                RepairState::BrokenPart {
                    part: RepairPartKind::Head,
                    ..
                } | RepairState::ConsumedPart { .. }
            )
        })
        .count();

    assert_eq!(final_toy_count, data.config.toy_count);
    assert_eq!(session.toys.len(), data.config.toy_count + 1);
    assert_eq!(session.displays.len(), data.displays.len());
    assert_eq!(session.completed_display_count(), 0);
}

#[test]
fn new_session_starts_with_one_split_robot() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    let broken_parts: Vec<&ToyState> = session
        .toys
        .iter()
        .filter(|toy| toy.is_repair_part())
        .collect();

    assert_eq!(broken_parts.len(), 2);
    assert!(broken_parts
        .iter()
        .any(|toy| toy.repair_part_kind() == Some(RepairPartKind::Head)));
    assert!(broken_parts
        .iter()
        .any(|toy| toy.repair_part_kind() == Some(RepairPartKind::Body)));
    for part in broken_parts {
        assert!(!data
            .displays
            .iter()
            .any(|display| toy_matches_display(part, display)));
    }
}

#[test]
fn new_session_generates_identity_variety_per_display() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    for display in &data.displays {
        let labels: HashSet<&str> = session
            .toys
            .iter()
            .filter(|toy| toy_matches_display(toy, display))
            .map(|toy| toy_profile(toy.category, toy.slot_number).label)
            .collect();

        assert!(labels.len() >= 5, "{} identities: {:?}", display.id, labels);
    }
}

#[test]
fn new_session_gives_loose_toys_varied_spawn_poses() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let tumbled_count = session
        .toys
        .iter()
        .filter(|toy| toy.spawn_pose.is_tumbled())
        .count();
    let upside_down_count = session
        .toys
        .iter()
        .filter(|toy| (toy.spawn_pose.roll.abs() - std::f32::consts::PI).abs() < 0.01)
        .count();
    let side_count = session
        .toys
        .iter()
        .filter(|toy| {
            (toy.spawn_pose.roll.abs() - std::f32::consts::FRAC_PI_2).abs() < 0.01
                || (toy.spawn_pose.pitch.abs() - std::f32::consts::FRAC_PI_2).abs() < 0.01
        })
        .count();

    assert!(tumbled_count > session.toys.len() / 2);
    assert!(upside_down_count > 0);
    assert!(side_count > 0);
}

#[test]
fn toy_colors_are_not_locked_to_display_category() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    for display in &data.displays {
        let colors: HashSet<(u8, u8, u8)> = session
            .toys
            .iter()
            .filter(|toy| toy_matches_display(toy, display))
            .map(|toy| {
                let color = toy_color(toy);
                (
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                )
            })
            .collect();

        assert!(
            colors.len() >= 5,
            "{} used only {:?} colors",
            display.id,
            colors
        );
    }
}

#[test]
fn new_session_starts_with_all_toys_on_the_floor() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    for toy in &session.toys {
        assert!(!toy.is_held);
        assert!(toy.placed_display_id.is_none());
        assert!(toy.placed_slot_index.is_none());
        for display in &data.displays {
            let on_display = toy.position.x >= display.x
                && toy.position.x <= display.x + display.w
                && toy.position.y >= display.y
                && toy.position.y <= display.y + display.h;
            assert!(
                !on_display,
                "{} started on display {} at {}, {}",
                toy.id, display.id, toy.position.x, toy.position.y
            );
        }
    }
}

#[test]
fn pickup_uses_crosshair_target_instead_of_closest_toy() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let aimed_toy_id = session.toys[1].id.clone();

    session.player.position = WorldPoint { x: 5.0, y: 5.0 };
    session.player.yaw = 0.0;
    session.player.pitch = -0.50;

    for toy in &mut session.toys {
        toy.position = WorldPoint { x: 16.8, y: 11.2 };
        toy.is_held = false;
        toy.placed_display_id = None;
        toy.placed_slot_index = None;
    }

    session.toys[0].position = WorldPoint { x: 5.45, y: 5.68 };
    session.toys[1].position = WorldPoint { x: 6.30, y: 5.00 };
    session.spatial.rebuild(&session.toys);

    let result = session.interact(&data);

    assert!(matches!(result, InteractionResult::PickedUp { .. }));
    assert_eq!(
        session.active_toy().map(|toy| toy.id.as_str()),
        Some(aimed_toy_id.as_str())
    );
}

#[test]
fn cannot_pick_up_second_toy_while_holding_one() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    let first_result = session.pick_up_toy(0);
    let second_result = session.pick_up_toy(1);

    assert!(matches!(first_result, InteractionResult::PickedUp { .. }));
    assert!(matches!(second_result, InteractionResult::InventoryFull));
    assert_eq!(session.player.carried_toy_ids.len(), 1);
    assert!(session.toys[0].is_held);
    assert!(!session.toys[1].is_held);
}

#[test]
fn interact_places_held_toy_on_floor_when_no_target_is_active() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let toy_id = session.toys[0].id.clone();

    session.pick_up_toy(0);
    session.player.position = WorldPoint { x: 9.0, y: 10.8 };
    session.player.yaw = 0.0;
    session.player.pitch = 0.0;

    let result = session.interact(&data);

    assert!(matches!(result, InteractionResult::Dropped { .. }));
    assert!(session.player.carried_toy_ids.is_empty());

    let dropped = session.toys.iter().find(|toy| toy.id == toy_id).unwrap();
    assert!(!dropped.is_held);
    assert!(dropped.placed_display_id.is_none());
    assert!(dropped.placed_slot_index.is_none());
    assert!(dropped.position.x > session.player.position.x + 0.7);
    assert!((dropped.position.y - session.player.position.y).abs() < 0.01);
}

#[test]
fn correct_placement_completes_a_display() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let display = &data.displays[0];
    let mut matching_toy_ids: Vec<(usize, String)> = session
        .toys
        .iter()
        .filter(|toy| toy_matches_display(toy, display))
        .map(|toy| (toy.slot_number, toy.id.clone()))
        .collect();
    matching_toy_ids.sort_by_key(|(slot_number, _)| *slot_number);

    for (slot_index, (_, toy_id)) in matching_toy_ids.into_iter().enumerate() {
        let toy_index = session
            .toys
            .iter()
            .position(|toy| toy.id == toy_id)
            .unwrap();
        session.pick_up_toy(toy_index);
        session.player.position = WorldPoint {
            x: display.x + display.w * 0.5,
            y: display.y + display.h * 0.5,
        };
        let _ = session.place_active_toy(0, slot_index, &data);
    }

    assert!(session.is_display_complete(&display.id));
    assert_eq!(session.completed_display_count(), 1);
}

#[test]
fn broken_part_must_be_repaired_before_display() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let robot_display_index = data
        .displays
        .iter()
        .position(|display| display.category == ToyCategory::ActionFigures)
        .unwrap();
    let body_index = session
        .toys
        .iter()
        .position(|toy| toy.repair_part_kind() == Some(RepairPartKind::Body))
        .unwrap();

    session.pick_up_toy(body_index);
    let result = session.place_active_toy(robot_display_index, 0, &data);

    assert!(matches!(result, InteractionResult::NeedsRepair { .. }));
    assert!(session.active_toy().unwrap().is_repair_part());
    assert_eq!(session.total_placed_toys(), 0);
}

#[test]
fn repair_bench_repairs_matching_benched_parts() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let robot_display = data
        .displays
        .iter()
        .find(|display| display.category == ToyCategory::ActionFigures)
        .unwrap();
    let body_id = session
        .toys
        .iter()
        .find(|toy| toy.repair_part_kind() == Some(RepairPartKind::Body))
        .unwrap()
        .id
        .clone();
    let head_id = session
        .toys
        .iter()
        .find(|toy| toy.repair_part_kind() == Some(RepairPartKind::Head))
        .unwrap()
        .id
        .clone();

    let bench = data.primary_bench();
    session.player.position = WorldPoint {
        x: bench.x,
        y: bench.y,
    };

    let body_index = session
        .toys
        .iter()
        .position(|toy| toy.id == body_id)
        .unwrap();
    session.pick_up_toy(body_index);
    let result = session.interact(&data);

    assert!(matches!(
        result,
        InteractionResult::PlacedOnRepairBench { .. }
    ));
    assert!(session.player.carried_toy_ids.is_empty());
    assert_eq!(
        session
            .toys
            .iter()
            .find(|toy| toy.id == body_id)
            .unwrap()
            .bench_slot_index,
        Some(0)
    );

    let head_index = session
        .toys
        .iter()
        .position(|toy| toy.id == head_id)
        .unwrap();
    session.pick_up_toy(head_index);
    let result = session.interact(&data);

    assert!(matches!(
        result,
        InteractionResult::PlacedOnRepairBench { .. }
    ));
    assert!(session.player.carried_toy_ids.is_empty());

    let result = session.interact(&data);

    assert!(matches!(result, InteractionResult::Repaired { .. }));
    assert_eq!(session.player.carried_toy_ids.len(), 1);
    let active_toy = session.active_toy().unwrap();
    assert_eq!(active_toy.id, body_id);
    assert!(!active_toy.is_repair_part());
    assert!(active_toy.bench_slot_index.is_none());
    assert!(toy_matches_display(active_toy, robot_display));
    assert!(session
        .toys
        .iter()
        .find(|toy| toy.id == head_id)
        .unwrap()
        .is_consumed_repair_part());
}

#[test]
fn tool_purchases_use_completed_display_credits() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    assert_eq!(session.carry_limit(&data.config), 1);
    assert!(!session.scanner_enabled());
    assert!(matches!(
        session.purchase_tool(&data, "toy_scanner"),
        ToolPurchaseResult::Locked { .. }
    ));

    complete_display_by_index(&mut session, &data, 0);
    assert_eq!(session.available_tool_credits(&data), 1);

    let result = session.purchase_tool(&data, "toy_scanner");
    assert!(matches!(
        result,
        ToolPurchaseResult::Purchased {
            ref tool_name,
            remaining_credits: 0,
        } if tool_name == "Toy Scanner"
    ));
    assert!(session.scanner_enabled());
    assert_eq!(session.carry_limit(&data.config), 1);

    complete_display_by_index(&mut session, &data, 1);
    assert_eq!(session.available_tool_credits(&data), 1);
    assert!(matches!(
        session.purchase_tool(&data, "small_trolley"),
        ToolPurchaseResult::NoToolsAvailable
    ));
    assert_eq!(session.carry_limit(&data.config), 1);
}

#[test]
fn placement_uses_target_slot() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let display = &data.displays[0];
    let toy_id = session
        .toys
        .iter()
        .find(|toy| toy_matches_display(toy, display) && toy.slot_number == 4)
        .unwrap()
        .id
        .clone();
    let toy_index = session
        .toys
        .iter()
        .position(|toy| toy.id == toy_id)
        .unwrap();

    session.pick_up_toy(toy_index);
    session.player.position = WorldPoint {
        x: display.x + display.w * 0.5,
        y: display.y + display.h * 0.5,
    };
    let _ = session.place_active_toy(0, 0, &data);

    let placed = session.toys.iter().find(|toy| toy.id == toy_id).unwrap();
    let expected = display_slot_position(display, 0, data.config.room_width);
    assert_eq!(placed.placed_slot_index, Some(0));
    assert!((placed.position.x - expected.x).abs() < f32::EPSILON);
    assert!((placed.position.y - expected.y).abs() < f32::EPSILON);
}

#[test]
fn wrong_placement_still_places_toy_and_counts_mistake() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let display = &data.displays[0];
    let toy_index = session
        .toys
        .iter()
        .position(|toy| !toy_matches_display(toy, display))
        .unwrap();
    let toy_id = session.toys[toy_index].id.clone();

    session.pick_up_toy(toy_index);
    let _ = session.place_active_toy(0, 0, &data);

    let placed = session.toys.iter().find(|toy| toy.id == toy_id).unwrap();
    assert_eq!(
        placed.placed_display_id.as_deref(),
        Some(display.id.as_str())
    );
    assert!(placed.wrong_marker_seconds > 0.0);
    assert_eq!(session.player.mistakes, 1);

    session.update_timer(GameSession::WRONG_MARKER_SECONDS + 0.1);
    let placed = session.toys.iter().find(|toy| toy.id == toy_id).unwrap();
    assert_eq!(placed.wrong_marker_seconds, 0.0);
}

#[test]
fn cannot_place_into_filled_slot() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let display = &data.displays[0];
    let toy_ids: Vec<String> = session
        .toys
        .iter()
        .filter(|toy| toy_matches_display(toy, display))
        .take(2)
        .map(|toy| toy.id.clone())
        .collect();

    let first_index = session
        .toys
        .iter()
        .position(|toy| toy.id == toy_ids[0])
        .unwrap();
    session.pick_up_toy(first_index);
    let _ = session.place_active_toy(0, 0, &data);

    let second_index = session
        .toys
        .iter()
        .position(|toy| toy.id == toy_ids[1])
        .unwrap();
    session.pick_up_toy(second_index);
    let result = session.place_active_toy(0, 0, &data);

    assert!(matches!(result, InteractionResult::ShelfSlotUnavailable));
    assert!(
        session
            .toys
            .iter()
            .find(|toy| toy.id == toy_ids[1])
            .unwrap()
            .is_held
    );
    assert_eq!(
        session
            .placed_toys_for_display(&display.id)
            .filter(|toy| toy.placed_slot_index == Some(0))
            .count(),
        1
    );
}

#[test]
fn wrong_toys_do_not_complete_display() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let display = &data.displays[0];
    let wrong_toy_ids: Vec<String> = session
        .toys
        .iter()
        .filter(|toy| !toy_matches_display(toy, display))
        .take(display.capacity)
        .map(|toy| toy.id.clone())
        .collect();

    for (slot_index, toy_id) in wrong_toy_ids.into_iter().enumerate() {
        let toy_index = session
            .toys
            .iter()
            .position(|toy| toy.id == toy_id)
            .unwrap();
        session.pick_up_toy(toy_index);
        let _ = session.place_active_toy(0, slot_index, &data);
    }

    assert_eq!(session.total_placed_toys(), display.capacity);
    assert!(!session.is_display_complete(&display.id));
}

#[test]
fn full_shelf_rejects_extra_toy() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let display = &data.displays[0];
    let mut matching_toy_ids: Vec<(usize, String)> = session
        .toys
        .iter()
        .filter(|toy| toy_matches_display(toy, display))
        .map(|toy| (toy.slot_number, toy.id.clone()))
        .collect();
    matching_toy_ids.sort_by_key(|(slot_number, _)| *slot_number);

    for (slot_index, (_, toy_id)) in matching_toy_ids.into_iter().enumerate() {
        let toy_index = session
            .toys
            .iter()
            .position(|toy| toy.id == toy_id)
            .unwrap();
        session.pick_up_toy(toy_index);
        let _ = session.place_active_toy(0, slot_index, &data);
    }

    let extra_toy_index = session
        .toys
        .iter()
        .position(|toy| !toy_matches_display(toy, display))
        .unwrap();
    session.pick_up_toy(extra_toy_index);
    let result = session.place_active_toy(0, 0, &data);

    assert!(matches!(result, InteractionResult::ShelfFull));
}

#[test]
fn can_pick_up_toy_from_display_slot() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let display = &data.displays[0];
    let toy_index = session
        .toys
        .iter()
        .position(|toy| !toy_matches_display(toy, display))
        .unwrap();
    let toy_id = session.toys[toy_index].id.clone();

    session.pick_up_toy(toy_index);
    let _ = session.place_active_toy(0, 0, &data);
    session.player.position = display_slot_position(display, 0, data.config.room_width);
    session.player.yaw = 0.0;

    let result = session.interact(&data);

    assert!(matches!(result, InteractionResult::PickedUp { .. }));
    let picked_up = session.toys.iter().find(|toy| toy.id == toy_id).unwrap();
    assert!(picked_up.is_held);
    assert!(picked_up.placed_display_id.is_none());
    assert!(picked_up.placed_slot_index.is_none());
    assert_eq!(session.total_placed_toys(), 0);
}

#[test]
fn movement_follows_player_yaw() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let start = session.player.position;

    session.player.yaw = 0.0;
    session.move_player(vec2(0.0, 1.0), &data, 0.25);

    assert!(session.player.position.x > start.x);
    assert!((session.player.position.y - start.y).abs() < 0.001);
}

#[test]
fn player_collides_with_aisle_shelving() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let shelf = &data.layout.shelving[0];

    session.player.position = WorldPoint {
        x: shelf.x + shelf.w * 0.5,
        y: shelf.y - 1.0,
    };
    session.player.yaw = std::f32::consts::FRAC_PI_2;
    for _ in 0..40 {
        session.move_player(vec2(0.0, 1.0), &data, 0.1);
    }

    let final_y = session.player.position.y;
    assert!(
        final_y > shelf.y - 1.0 + 0.2,
        "player should approach shelf"
    );
    assert!(
        final_y < shelf.y - 0.40,
        "player should stop at the shelf edge, got y {final_y}"
    );
}

#[test]
fn look_yaw_is_not_clamped_to_one_turn() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.player.yaw = 0.0;

    session.update_player_look(std::f32::consts::TAU * 2.25, 0.0);

    assert!(session.player.yaw > std::f32::consts::TAU * 2.0);
}

fn complete_display_by_index(session: &mut GameSession, data: &GameData, display_index: usize) {
    let display = &data.displays[display_index];
    let mut matching_toy_ids: Vec<(usize, String)> = session
        .toys
        .iter()
        .filter(|toy| toy_matches_display(toy, display))
        .map(|toy| (toy.slot_number, toy.id.clone()))
        .collect();
    matching_toy_ids.sort_by_key(|(slot_number, _)| *slot_number);

    for (slot_index, (_, toy_id)) in matching_toy_ids.into_iter().enumerate() {
        let toy_index = session
            .toys
            .iter()
            .position(|toy| toy.id == toy_id)
            .unwrap();
        session.pick_up_toy(toy_index);
        let _ = session.place_active_toy(display_index, slot_index, data);
    }
}

#[test]
fn spatial_grid_tracks_pickup_place_and_drop() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    let toy_index = session
        .toys
        .iter()
        .position(|toy| !toy.is_repair_part() && toy.placed_display_id.is_none())
        .unwrap();
    let spawn = session.toys[toy_index].position.to_vec2();
    assert!(session
        .spatial
        .indices_near(spawn, 0.1)
        .contains(&toy_index));

    session.pick_up_toy(toy_index);
    assert!(!session
        .spatial
        .indices_near(spawn, 0.1)
        .contains(&toy_index));

    let _ = session.place_active_toy(0, 0, &data);
    let placed = session.toys[toy_index].position.to_vec2();
    assert!(session
        .spatial
        .indices_near(placed, 0.1)
        .contains(&toy_index));

    session.pick_up_toy(toy_index);
    session.drop_active(&data).unwrap();
    let dropped = session.toys[toy_index].position.to_vec2();
    assert!(session
        .spatial
        .indices_near(dropped, 0.1)
        .contains(&toy_index));
}

#[test]
fn parts_bench_at_the_nearest_bench() {
    let data = GameData::load().unwrap();
    assert!(data.layout.benches.len() >= 2, "expected multiple benches");
    let second_bench = &data.layout.benches[1];
    let mut session = GameSession::new(&data);

    let head_index = session
        .toys
        .iter()
        .position(|toy| toy.repair_part_kind() == Some(RepairPartKind::Head))
        .unwrap();
    session.pick_up_toy(head_index);
    session.player.position = WorldPoint {
        x: second_bench.x,
        y: second_bench.y,
    };

    let result = session.interact(&data);

    assert!(matches!(
        result,
        InteractionResult::PlacedOnRepairBench { .. }
    ));
    assert_eq!(
        session.toys[head_index].bench_id.as_deref(),
        Some(second_bench.id.as_str())
    );
    assert!(session.toys[head_index].bench_slot_index.is_some());
}
