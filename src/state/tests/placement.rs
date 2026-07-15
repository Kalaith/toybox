use super::*;

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

    for (slot_index, (_, toy_id)) in matching_toy_ids
        .into_iter()
        .take(display.capacity)
        .enumerate()
    {
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
        .position(|toy| !toy_matches_display(toy, display) && !toy.is_repair_part())
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
        .filter(|toy| !toy_matches_display(toy, display) && !toy.is_repair_part())
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

    for (slot_index, (_, toy_id)) in matching_toy_ids
        .into_iter()
        .take(display.capacity)
        .enumerate()
    {
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
        .position(|toy| !toy_matches_display(toy, display) && !toy.is_repair_part())
        .unwrap();
    session.pick_up_toy(extra_toy_index);
    let result = session.place_active_toy(0, 0, &data);

    assert!(matches!(result, InteractionResult::ShelfFull));
}
