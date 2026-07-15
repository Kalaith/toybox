use super::*;

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
    let (body_id, pair_repair_id) = session
        .toys
        .iter()
        .find_map(|toy| match &toy.repair_state {
            RepairState::BrokenPart {
                repair_id,
                part: RepairPartKind::Body,
                ..
            } => Some((toy.id.clone(), repair_id.clone())),
            _ => None,
        })
        .unwrap();
    let head_id = session
        .toys
        .iter()
        .find(|toy| {
            matches!(
                &toy.repair_state,
                RepairState::BrokenPart {
                    repair_id,
                    part: RepairPartKind::Head,
                    ..
                } if *repair_id == pair_repair_id
            )
        })
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
    let home_display = data
        .displays
        .iter()
        .find(|display| display.category == active_toy.category)
        .unwrap();
    assert!(toy_matches_display(active_toy, home_display));
    assert!(session
        .toys
        .iter()
        .find(|toy| toy.id == head_id)
        .unwrap()
        .is_consumed_repair_part());
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

fn complete_display_by_index(session: &mut GameSession, data: &GameData, display_index: usize) {
    let display = &data.displays[display_index];
    let mut matching_toy_ids: Vec<(usize, String)> = session
        .toys
        .iter()
        .filter(|toy| {
            // Same-theme displays share matching toys: leave already-shelved
            // ones on their display.
            toy_matches_display(toy, display) && toy.placed_display_id.is_none() && !toy.is_held
        })
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
        let _ = session.place_active_toy(display_index, slot_index, data);
    }
}
