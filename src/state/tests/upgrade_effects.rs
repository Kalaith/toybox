//! Each tool in `upgrades.json` has to actually change something. These lock
//! the effect wiring, not the balance numbers — the JSON is free to retune.

use super::*;

fn grant(session: &mut GameSession, upgrade_id: &str) {
    session.unlocked_upgrade_ids.push(upgrade_id.to_owned());
}

#[test]
fn an_unequipped_shop_uses_the_configured_baseline() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    assert_eq!(session.carry_limit(&data), data.config.starting_carry_limit);
    assert_eq!(session.speed_multiplier(&data), 1.0);
    assert_eq!(
        session.interaction_reach(&data),
        data.config.interaction_radius
    );
    assert_eq!(session.forgiven_mistakes(&data), 0);
    assert!(!session.scanner_enabled(&data));
}

#[test]
fn every_tool_moves_the_value_it_advertises() {
    let data = GameData::load().unwrap();
    let baseline = GameSession::new(&data);

    for upgrade in &data.upgrades {
        let mut session = GameSession::new(&data);
        grant(&mut session, &upgrade.id);

        let moved = session.carry_limit(&data) != baseline.carry_limit(&data)
            || session.speed_multiplier(&data) != baseline.speed_multiplier(&data)
            || session.interaction_reach(&data) != baseline.interaction_reach(&data)
            || session.forgiven_mistakes(&data) != baseline.forgiven_mistakes(&data)
            || session.scanner_enabled(&data) != baseline.scanner_enabled(&data);
        assert!(moved, "{} changed nothing when owned", upgrade.id);
    }
}

#[test]
fn the_trolley_lets_the_player_hold_more_than_one_toy() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    grant(&mut session, "sorting_trolley");
    let limit = session.carry_limit(&data);
    assert!(limit > 1, "the trolley should raise the carry limit");

    let loose: Vec<usize> = session
        .toys
        .iter()
        .enumerate()
        .filter(|(_, toy)| !toy.is_held && toy.placed_display_id.is_none())
        .map(|(index, _)| index)
        .take(limit + 1)
        .collect();

    for &toy_index in loose.iter().take(limit) {
        assert!(matches!(
            session.pick_up_toy(toy_index, &data),
            InteractionResult::PickedUp { .. }
        ));
    }
    assert_eq!(session.player.carried_toy_ids.len(), limit);

    // One past the limit is refused rather than silently dropping a toy.
    assert!(matches!(
        session.pick_up_toy(loose[limit], &data),
        InteractionResult::InventoryFull
    ));
    assert_eq!(session.player.carried_toy_ids.len(), limit);
}

#[test]
fn a_save_made_with_the_trolley_keeps_its_load_across_a_reload() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    grant(&mut session, "sorting_trolley");
    let limit = session.carry_limit(&data);

    let loose: Vec<usize> = session
        .toys
        .iter()
        .enumerate()
        .filter(|(_, toy)| !toy.is_held && toy.placed_display_id.is_none())
        .map(|(index, _)| index)
        .take(limit)
        .collect();
    for toy_index in loose {
        session.pick_up_toy(toy_index, &data);
    }

    let reloaded = GameSession::from_save(session.to_save("3.0.0"), &data);

    // Load-time normalisation trims to the carry limit; with the trolley owned
    // that limit is the trolley's, not the bare-handed one.
    assert_eq!(reloaded.player.carried_toy_ids.len(), limit);
    assert!(reloaded
        .toys
        .iter()
        .filter(|toy| toy.is_held)
        .count()
        .eq(&limit));
}

#[test]
fn forgiveness_waives_the_clock_but_still_records_the_mistake() {
    let data = GameData::load().unwrap();
    let display = &data.displays[0];

    let mut plain = GameSession::new(&data);
    let mut forgiven = GameSession::new(&data);
    grant(&mut forgiven, "managers_nod");
    assert!(forgiven.forgiven_mistakes(&data) > 0);

    for session in [&mut plain, &mut forgiven] {
        let wrong = session
            .toys
            .iter()
            .position(|toy| {
                !toy.is_repair_part() && !toy.is_held && !toy_matches_display(toy, display)
            })
            .unwrap();
        session.pick_up_toy(wrong, &data);
        session.place_active_toy(0, 0, &data);
    }

    assert_eq!(plain.player.mistakes, 1);
    assert_eq!(forgiven.player.mistakes, 1, "the mistake is still counted");
    assert!(plain.player.elapsed_seconds > 0.0);
    assert_eq!(
        forgiven.player.elapsed_seconds, 0.0,
        "forgiveness should cost no time"
    );
}

/// What the "carry is full" message names is true, and comes from the data.
///
/// It used to read "Sorting cart is full" — a name the game does not use (the
/// tool is the Sorting *Trolley*, and "Cart Blocks" is an unrelated toy), and
/// it fired at the *starting* carry limit of one, so a player who had never
/// bought a tool was told a tool of theirs was full. The HUD prompt for the
/// same state said "Carry full" and was right all along; only the notification
/// was wrong, and the two channels disagreed.
#[test]
fn the_carry_full_message_names_a_tool_the_player_owns() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);

    assert_eq!(
        session.carry_tool_name(&data),
        None,
        "bare-handed, nothing but hands is full"
    );
    // ...and the limit really does bite there, which is why it mattered.
    assert_eq!(session.carry_limit(&data), data.config.starting_carry_limit);

    let trolley = data
        .upgrades
        .iter()
        .find(|upgrade| {
            matches!(
                upgrade.effect,
                crate::data::UpgradeEffect::CarryLimit { .. }
            )
        })
        .expect("a carry tool");
    session.unlocked_upgrade_ids.push(trolley.id.clone());

    assert_eq!(
        session.carry_tool_name(&data),
        Some(trolley.name.as_str()),
        "the message must use the name in upgrades.json, not a copy of it"
    );
}

/// Prices read as English, and every place that quotes one agrees.
///
/// The four sites that name a price — the shop row, its refusal line, the HUD
/// nudge and the Buy-refused notification — all wrote `credit(s)`. The Toy
/// Scanner costs exactly 1 and is the first tool anyone buys, so `Costs 1
/// credit(s)` sat on the very first purchase decision in the game. Routed
/// through one helper so the four cannot drift apart later.
#[test]
fn a_price_of_one_reads_as_one_credit() {
    use crate::ui::credits_phrase;

    assert_eq!(credits_phrase(1), "1 credit");
    assert_eq!(credits_phrase(0), "0 credits");
    assert_eq!(credits_phrase(2), "2 credits");
    assert_eq!(credits_phrase(11), "11 credits");

    // The singular is reachable from the shipped data, which is the only
    // reason this matters.
    let data = GameData::load().unwrap();
    assert!(
        data.upgrades.iter().any(|upgrade| upgrade.cost == 1),
        "no tool costs 1, so the singular would never be seen"
    );
}
