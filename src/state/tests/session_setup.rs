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

    let head_count = session
        .toys
        .iter()
        .filter(|toy| toy.repair_part_kind() == Some(RepairPartKind::Head))
        .count();

    assert_eq!(final_toy_count, data.config.toy_count);
    assert_eq!(session.toys.len(), data.config.toy_count + head_count);
    assert_eq!(session.displays.len(), data.displays.len());
    assert_eq!(session.completed_display_count(), 0);
}

#[test]
fn new_session_breaks_a_deterministic_fraction_into_cross_zone_pairs() {
    let data = GameData::load().unwrap();
    let session = GameSession::new(&data);

    let mut bodies: std::collections::HashMap<&str, &ToyState> = std::collections::HashMap::new();
    let mut heads: std::collections::HashMap<&str, &ToyState> = std::collections::HashMap::new();
    for toy in &session.toys {
        if let RepairState::BrokenPart {
            repair_id, part, ..
        } = &toy.repair_state
        {
            match part {
                RepairPartKind::Body => bodies.insert(repair_id.as_str(), toy),
                RepairPartKind::Head => heads.insert(repair_id.as_str(), toy),
            };
            assert!(!data
                .displays
                .iter()
                .any(|display| toy_matches_display(toy, display)));
        }
    }

    // Roughly broken_fraction of the store, every body with exactly one head.
    let expected = (data.config.toy_count as f32 * data.config.broken_fraction) as usize;
    assert_eq!(bodies.len(), heads.len());
    assert!(
        bodies.len() * 5 >= expected * 4 && bodies.len() * 5 <= expected * 6,
        "broken count {} far from expected {expected}",
        bodies.len()
    );

    for (repair_id, body) in &bodies {
        let head = heads
            .get(repair_id)
            .unwrap_or_else(|| panic!("no head for {repair_id}"));
        let body_zone = data.layout.zone_name_at(body.position.x, body.position.y);
        let head_zone = data.layout.zone_name_at(head.position.x, head.position.y);
        assert_ne!(
            body_zone, head_zone,
            "parts of {repair_id} landed in the same zone"
        );
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

/// The three render bands are all reachable.
///
/// `scene3d::draw_loose_toys` picks a stand-in cube beyond `toy_lod_distance`,
/// a full upright model beyond `toy_pose_distance`, and a full posed model
/// closer than that. The middle band is written as an `else if`, so setting the
/// two distances *equal* deletes it silently — which is how the game shipped:
/// both were 5.0, the "full detail, but upright" arm could never run, and every
/// toy past five metres was a coloured box. That LOD was tuned when the shop
/// held 4000 toys; it holds 240 now.
///
/// Measured on the shipped shop with the vsync-uncapped bench, average fps:
/// lod 5 -> ~320, 7 -> 272, 8 -> 245, 9 -> 221, 12 (no cubes at all) -> 113.
/// 8.0 buys nearly all of the visual gain for about 1.3x the frame cost.
#[test]
fn every_toy_render_band_is_reachable() {
    let data = GameData::load().unwrap();
    let config = &data.config;

    assert!(
        config.toy_lod_distance > config.toy_pose_distance,
        "lod {} is not beyond pose {}, so the upright band is dead code",
        config.toy_lod_distance,
        config.toy_pose_distance
    );
    assert!(
        config.toy_render_distance > config.toy_lod_distance,
        "render {} is not beyond lod {}, so no toy is ever a stand-in",
        config.toy_render_distance,
        config.toy_lod_distance
    );
    // Toys inside this radius always draw, so it must not reach past the point
    // where they stop being drawn at all.
    assert!(config.toy_always_draw_radius <= config.toy_render_distance);
}
