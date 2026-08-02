use super::*;

/// Total zone capacity must account for every toy in the shop, or the HUD
/// percentages are measured against a denominator that quietly loses displays.
#[test]
fn every_display_lands_in_exactly_one_zone() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let progress = session.zone_progress(&data);

    assert_eq!(progress.len(), data.layout.zones.len());
    let total_capacity: usize = progress.iter().map(|zone| zone.capacity).sum();
    assert_eq!(total_capacity, data.config.toy_count);
}

#[test]
fn a_fresh_shop_reads_zero_everywhere_it_has_shelves() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    for zone in session.zone_progress(&data) {
        if zone.has_displays() {
            assert_eq!(zone.placed, 0);
            assert_eq!(zone.fraction(), 0.0);
        } else {
            // Nothing to tidy reads as done, not as permanently 0% complete.
            assert_eq!(zone.fraction(), 1.0);
        }
    }
}

#[test]
fn shelving_a_display_moves_only_its_own_zone() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let before = session.zone_progress(&data);

    complete_display_by_index(&mut session, &data, 0);
    let after = session.zone_progress(&data);

    let moved: Vec<usize> = before
        .iter()
        .zip(&after)
        .filter(|(before, after)| before.placed != after.placed)
        .map(|(_, after)| after.zone_index)
        .collect();
    assert_eq!(moved.len(), 1, "one display should move one zone");

    let zone = after[moved[0]];
    assert_eq!(zone.placed, data.displays[0].capacity);
    assert!(zone.fraction() > 0.0 && zone.fraction() <= 1.0);
}

#[test]
fn wrongly_shelved_toys_do_not_count_toward_a_zone() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    // A toy whose category does not match display 0, shelved there anyway.
    let display = &data.displays[0];
    let wrong_index = session
        .toys
        .iter()
        .position(|toy| !toy.is_repair_part() && !toy.is_held && !toy_matches_display(toy, display))
        .unwrap();
    session.pick_up_toy(wrong_index, &data);
    session.place_active_toy(0, 0, &data);

    let placed: usize = session
        .zone_progress(&data)
        .iter()
        .map(|zone| zone.placed)
        .sum();
    assert_eq!(placed, 0, "a mis-shelved toy must not read as progress");
}

fn complete_display_by_index(session: &mut GameSession, data: &GameData, display_index: usize) {
    let display = &data.displays[display_index];
    let mut matching: Vec<(usize, String)> = session
        .toys
        .iter()
        .filter(|toy| {
            toy_matches_display(toy, display) && toy.placed_display_id.is_none() && !toy.is_held
        })
        .map(|toy| (toy.slot_number, toy.id.clone()))
        .collect();
    matching.sort_by_key(|(slot_number, _)| *slot_number);

    for (slot_index, (_, toy_id)) in matching.into_iter().take(display.capacity).enumerate() {
        let toy_index = session
            .toys
            .iter()
            .position(|toy| toy.id == toy_id)
            .unwrap();
        session.pick_up_toy(toy_index, data);
        let _ = session.place_active_toy(display_index, slot_index, data);
    }
}
