//! Deterministic toy generation and scatter placement for a fresh run.

use super::collision::keep_off_fixtures;
use super::{repair, RepairState, ToyState, WorldPoint};
use crate::data::GameData;
use crate::toys::{spawn_pose_for_toy, toy_name};
use macroquad::prelude::*;

pub(super) fn build_toys(data: &GameData) -> Vec<ToyState> {
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
                color_index: (display_index + slot_index) % 5,
                position: scattered_position(toy_index, display_index, slot_index, data),
                spawn_pose: spawn_pose_for_toy(toy_index, display_index, slot_index),
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

    repair::split_initial_broken_toys(&mut toys, data);

    toys
}

fn scattered_position(
    toy_index: usize,
    display_index: usize,
    slot_index: usize,
    data: &GameData,
) -> WorldPoint {
    let config = &data.config;
    let (anchor, radius) = mess_pile_anchor(toy_index, display_index, slot_index, data);
    let angle = toy_index as f32 * 2.399 + display_index as f32 * 0.77 + slot_index as f32 * 0.19;
    let ring_seed = ((toy_index * 37 + display_index * 11 + slot_index * 17) % 100) as f32 / 100.0;
    let spill = if toy_index.is_multiple_of(11) {
        1.38
    } else {
        1.0
    };
    let squash = 0.56 + ((toy_index * 7 + slot_index * 5) % 30) as f32 / 100.0;
    let offset = vec2(
        angle.cos() * radius * ring_seed * spill,
        angle.sin() * radius * squash,
    );
    let jitter = vec2(
        (((toy_index * 41) % 23) as f32 - 11.0) * 0.018,
        (((toy_index * 59) % 29) as f32 - 14.0) * 0.016,
    );
    let position = keep_off_fixtures(anchor + offset + jitter, data);

    WorldPoint {
        x: position.x.clamp(0.8, config.room_width - 0.8),
        y: position.y.clamp(0.8, config.room_height - 0.8),
    }
}

fn mess_pile_anchor(
    toy_index: usize,
    display_index: usize,
    slot_index: usize,
    data: &GameData,
) -> (Vec2, f32) {
    let piles = &data.layout.scatter_piles;
    let total_weight: usize = piles.iter().map(|pile| pile.weight).sum();
    let pile_slot = (toy_index * 7 + display_index * 3 + slot_index) % total_weight.max(1);

    let mut cursor = 0;
    for pile in piles {
        cursor += pile.weight;
        if pile_slot < cursor {
            return (vec2(pile.x, pile.y), pile.radius);
        }
    }
    let last = piles.last().expect("layout load validates scatter piles");
    (vec2(last.x, last.y), last.radius)
}
