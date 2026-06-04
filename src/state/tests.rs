use super::*;
use crate::toys::{toy_color, toy_profile};
use std::collections::HashSet;

#[test]
fn new_session_generates_requested_toys() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(session.toys.len(), data.config.toy_count);
    assert_eq!(session.displays.len(), data.displays.len());
    assert_eq!(session.completed_display_count(), 0);
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

        assert_eq!(labels.len(), 5, "{} identities: {:?}", display.id, labels);
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
    session.move_player(vec2(0.0, 1.0), &data.config, 0.25);

    assert!(session.player.position.x > start.x);
    assert!((session.player.position.y - start.y).abs() < 0.001);
}

#[test]
fn look_yaw_is_not_clamped_to_one_turn() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.player.yaw = 0.0;

    session.update_player_look(std::f32::consts::TAU * 2.25, 0.0);

    assert!(session.player.yaw > std::f32::consts::TAU * 2.0);
}
