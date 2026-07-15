use super::*;

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
fn can_pick_up_toy_from_display_slot() {
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
