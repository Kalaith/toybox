use super::*;

#[test]
fn new_session_generates_requested_toys() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(session.toys.len(), data.config.toy_count);
    assert_eq!(session.displays.len(), data.displays.len());
    assert_eq!(session.completed_display_count(), 0);
}

#[test]
fn new_session_starts_with_all_toys_on_the_floor() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    for toy in &session.toys {
        assert!(!toy.is_held);
        assert!(toy.placed_display_id.is_none());
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
    let matching_toy_ids: Vec<String> = session
        .toys
        .iter()
        .filter(|toy| toy_matches_display(toy, display))
        .map(|toy| toy.id.clone())
        .collect();

    for toy_id in matching_toy_ids {
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
        let _ = session.place_active_toy(0, &data);
    }

    assert!(session.is_display_complete(&display.id));
    assert_eq!(session.completed_display_count(), 1);
}

#[test]
fn placement_uses_toy_slot_number() {
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
    let _ = session.place_active_toy(0, &data);

    let placed = session.toys.iter().find(|toy| toy.id == toy_id).unwrap();
    let expected = display_slot_position(display, 3, data.config.room_width);
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
    let _ = session.place_active_toy(0, &data);

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
