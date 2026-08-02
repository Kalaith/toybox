//! Audit of the load path: every shape of save the game has ever written, plus
//! the shapes it never wrote but could be handed, must either load or degrade
//! to a fresh session. Nothing here may panic.

use super::*;
use serde_json::{json, Value};

/// Strip every field added after the pre-expansion release, leaving the save
/// shaped the way an old build wrote it.
fn strip_post_expansion_fields(mut save: Value) -> Value {
    for toy in save["toys"].as_array_mut().unwrap() {
        let toy = toy.as_object_mut().unwrap();
        for field in [
            "spawn_pose",
            "placed_slot_index",
            "bench_slot_index",
            "bench_id",
            "wrong_marker_seconds",
            "repair_state",
        ] {
            toy.remove(field);
        }
    }
    let player = save["player"].as_object_mut().unwrap();
    player.remove("yaw");
    player.remove("pitch");
    save
}

#[test]
fn stale_save_with_wrong_toy_count_restocks_fresh() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.toys.truncate(100);
    let stale = serde_json::to_value(session.to_save("2.1.0")).unwrap();

    let migrated = migrate_save_value(Some("2.1.0".to_owned()), stale, &data).unwrap();

    let live_toys = migrated
        .toys
        .iter()
        .filter(|toy| {
            !toy.is_consumed_repair_part() && toy.repair_part_kind() != Some(RepairPartKind::Head)
        })
        .count();
    assert_eq!(live_toys, data.config.toy_count);
    assert_eq!(migrated.version, data.config.version);
}

#[test]
fn pre_expansion_save_loads_without_the_fields_it_never_had() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    // A pre-expansion store held no broken toys: drop the scattered heads and
    // let the stripped `repair_state` default the bodies back to whole. The
    // live count still has to match, or the restock path fires instead.
    session
        .toys
        .retain(|toy| toy.repair_part_kind() != Some(RepairPartKind::Head));
    assert_eq!(session.toys.len(), data.config.toy_count);
    let legacy =
        strip_post_expansion_fields(serde_json::to_value(session.to_save("2.1.0")).unwrap());

    let migrated = migrate_save_value(Some("2.1.0".to_owned()), legacy, &data).unwrap();
    let session = GameSession::from_save(migrated, &data);

    // Defaults filled in, and load-time repair gave every toy a real pose.
    assert_eq!(session.toys.len(), data.config.toy_count);
    assert!(session
        .toys
        .iter()
        .all(|toy| !toy.spawn_pose.is_uninitialized() || toy.placed_display_id.is_some()));
    assert!(session.player.yaw.is_finite());
    assert!(session.player.pitch.abs() <= GameSession::MAX_LOOK_PITCH);
}

#[test]
fn save_wrapped_in_a_data_envelope_is_unwrapped() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);
    let wrapped = json!({
        "version": "2.1.0",
        "data": serde_json::to_value(session.to_save("2.1.0")).unwrap(),
    });

    let migrated = migrate_save_value(Some("2.1.0".to_owned()), wrapped, &data).unwrap();

    assert_eq!(migrated.version, data.config.version);
    assert_eq!(migrated.toys.len(), session.toys.len());
}

#[test]
fn unreadable_saves_degrade_to_a_fresh_session() {
    let data = GameData::load().unwrap();

    for junk in [
        json!(null),
        json!("not a save"),
        json!(42),
        json!([]),
        json!({}),
        json!({ "toys": "not an array" }),
        // Right shape, required field missing.
        json!({ "version": "2.1.0", "player": {}, "toys": [] }),
    ] {
        let migrated = migrate_save_value(None, junk.clone(), &data)
            .unwrap_or_else(|error| panic!("{junk} should not error: {error}"));
        let live_toys = migrated
            .toys
            .iter()
            .filter(|toy| {
                !toy.is_consumed_repair_part()
                    && toy.repair_part_kind() != Some(RepairPartKind::Head)
            })
            .count();
        assert_eq!(live_toys, data.config.toy_count, "for {junk}");
    }
}

#[test]
fn a_save_carrying_a_toy_it_never_flagged_as_held_is_reconciled() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    let carried_id = session.toys[0].id.clone();

    // Both halves of the desync an older writer (or a hand-edited save) can
    // leave behind: carried but not held, and held but not carried.
    session.player.carried_toy_ids = vec![carried_id.clone()];
    session.player.active_carry_index = 0;
    session.toys[0].is_held = false;
    session.toys[1].is_held = true;

    let reloaded = GameSession::from_save(session.to_save("2.1.0"), &data);

    let carried = reloaded
        .toys
        .iter()
        .find(|toy| toy.id == carried_id)
        .unwrap();
    assert!(
        carried.is_held,
        "a toy in carried_toy_ids must be flagged held or it renders on the floor"
    );
    assert_eq!(
        reloaded.active_toy().map(|toy| toy.id.as_str()),
        Some(carried_id.as_str())
    );

    let stray = &reloaded.toys[1];
    assert!(
        !stray.is_held,
        "a held toy nobody is carrying is unreachable: pickup targeting skips held toys"
    );
}

#[test]
fn the_retired_tag_lantern_id_still_grants_the_scanner() {
    let data = GameData::load().unwrap();
    let mut session = GameSession::new(&data);
    session.unlocked_upgrade_ids = vec!["tag_lantern".to_owned()];

    let reloaded = GameSession::from_save(session.to_save("2.1.0"), &data);

    assert!(reloaded.scanner_enabled(&data));
    // The retired id must still spend its credit, or the tool comes out free.
    assert_eq!(reloaded.available_tool_credits(&data), 0);
}
