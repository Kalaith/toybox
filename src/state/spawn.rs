//! Deterministic toy generation and scatter placement for a fresh run.

use super::collision::keep_off_fixtures;
use super::{repair, seeded_offset, RepairState, ToyState, WorldPoint};
use crate::data::GameData;
use crate::toys::{spawn_pose_for_toy, toy_name};
use macroquad::prelude::*;

pub(super) fn build_toys(data: &GameData, shift_seed: u64) -> Vec<ToyState> {
    let mut toys = Vec::with_capacity(data.config.toy_count + 1);

    for (display_index, display) in data.displays.iter().enumerate() {
        for slot_index in 0..display.capacity {
            if toys.len() >= data.config.toy_count {
                break;
            }

            let toy_index = toys.len();
            let slot_number = slot_index + 1;
            toys.push(ToyState {
                id: format!("toy_{toy_index:03}"),
                name: toy_name(display, slot_number),
                category: display.category,
                theme: display.theme.clone(),
                slot_number,
                color_index: (display_index
                    + slot_index
                    + seeded_offset(shift_seed, toy_index as u64 ^ 0xC010_45E3, 5))
                    % 5,
                position: scattered_position(
                    toy_index,
                    display_index,
                    slot_index,
                    shift_seed,
                    data,
                ),
                spawn_pose: seeded_spawn_pose(toy_index, display_index, slot_index, shift_seed),
                is_held: false,
                placed_display_id: None,
                placed_slot_index: None,
                bench_slot_index: None,
                bench_id: None,
                wrong_marker_seconds: 0.0,
                repair_state: RepairState::Whole,
            });
        }
    }

    repair::split_initial_broken_toys(&mut toys, shift_seed, data);

    toys
}

/// Deterministic hash-spread over the whole floor: every fresh run drops the
/// same messy carpet of toys across all zones and aisles.
fn scattered_position(
    toy_index: usize,
    display_index: usize,
    slot_index: usize,
    shift_seed: u64,
    data: &GameData,
) -> WorldPoint {
    let config = &data.config;
    let hash_x = (toy_index
        .wrapping_mul(2_654_435_761)
        .wrapping_add(display_index.wrapping_mul(40_503))
        .wrapping_add(slot_index.wrapping_mul(9_973))
        .wrapping_add(seeded_offset(
            shift_seed,
            toy_index as u64 ^ 0x51CA_77E2,
            10_007,
        )))
        % 10_007;
    let hash_y = (toy_index
        .wrapping_mul(1_327_217_885)
        .wrapping_add(display_index.wrapping_mul(69_931))
        .wrapping_add(slot_index.wrapping_mul(28_657))
        .wrapping_add(seeded_offset(
            shift_seed,
            toy_index as u64 ^ 0xA115_EED5,
            10_007,
        )))
        % 10_007;

    let x = 0.8 + (hash_x as f32 / 10_007.0) * (config.room_width - 1.6);
    let y = 0.8 + (hash_y as f32 / 10_007.0) * (config.room_height - 1.6);
    let position = keep_off_fixtures(vec2(x, y), data);

    WorldPoint {
        x: position.x.clamp(0.8, config.room_width - 0.8),
        y: position.y.clamp(0.8, config.room_height - 0.8),
    }
}

fn seeded_spawn_pose(
    toy_index: usize,
    display_index: usize,
    slot_index: usize,
    shift_seed: u64,
) -> crate::toys::ToySpawnPose {
    // Keep the values modest because `spawn_pose_for_toy` uses ordinary
    // arithmetic internally; wrapping huge seed words into its indices would
    // overflow debug builds without adding useful visual variety.
    spawn_pose_for_toy(
        toy_index + seeded_offset(shift_seed, toy_index as u64 ^ 0x705E_0001, 360),
        display_index + seeded_offset(shift_seed, toy_index as u64 ^ 0x705E_0002, 64),
        slot_index + seeded_offset(shift_seed, toy_index as u64 ^ 0x705E_0003, 64),
    )
}
