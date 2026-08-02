use super::{
    display_slot_position, toy_matches_display, DisplaySlotTarget, GamePhase, GameSession,
    InteractionPreview, InteractionResult, ToyState, SINGLE_CARRY_LIMIT,
};
use crate::data::{DisplayDef, GameConfig, GameData};
use macroquad::prelude::*;

const SLOT_LOOK_MIN_DOT: f32 = 0.42;
const SLOT_LOOK_MAX_LATERAL: f32 = 1.15;
const LOOSE_TOY_LOOK_RADIUS: f32 = 0.58;
const PLAYER_EYE_HEIGHT: f32 = 1.08;
// Placed toys sit exactly on their slot position, so a small query radius
// only has to absorb grid-cell quantization.
const SLOT_OCCUPANCY_RADIUS: f32 = 0.25;

impl GameSession {
    pub fn interact(&mut self, data: &GameData) -> InteractionResult {
        if self.phase != GamePhase::Playing {
            return InteractionResult::NothingNearby;
        }

        if let Some(active_toy) = self.active_toy() {
            if active_toy.is_repair_part() {
                if self.is_near_repair_bench(data) {
                    return self.place_active_on_repair_bench(data);
                }
                if self.targeted_display_slot(data).is_some() || self.is_near_display(data) {
                    return InteractionResult::NeedsRepair {
                        toy_name: active_toy.name.clone(),
                    };
                }
                return self.drop_active_as_interaction(data);
            }
            if let Some(target) = self.targeted_empty_display_slot(data) {
                return self.place_active_toy(target.display_index, target.slot_index, data);
            }
            if let Some(target) = self.targeted_display_slot(data) {
                let display = &data.displays[target.display_index];
                if self.display_is_full(display, data.config.room_width) {
                    return InteractionResult::ShelfFull;
                }
                return InteractionResult::ShelfSlotUnavailable;
            }
            if self.is_near_display(data) {
                return InteractionResult::ShelfSlotUnavailable;
            }
            return self.drop_active_as_interaction(data);
        }

        if self.is_near_repair_bench(data) && self.benched_repair_name(data).is_some() {
            return self.repair_benched_toys(data);
        }

        if let Some(target) = self.targeted_display_slot(data) {
            let display = &data.displays[target.display_index];
            if let Some(toy_index) =
                self.toy_index_at_display_slot(display, target.slot_index, data.config.room_width)
            {
                if self.player.carried_toy_ids.len() >= self.carry_limit(&data.config) {
                    return InteractionResult::InventoryFull;
                }
                return self.pick_up_placed_toy(toy_index, data);
            }
        }

        if self.player.carried_toy_ids.len() >= self.carry_limit(&data.config) {
            return InteractionResult::InventoryFull;
        }

        if let Some(toy_index) = self.targeted_loose_toy_index(&data.config) {
            return self.pick_up_toy(toy_index);
        }

        if self.is_near_repair_bench(data) && self.repair_bench_is_full(data) {
            return InteractionResult::RepairMismatch;
        }

        InteractionResult::NothingNearby
    }

    pub fn interaction_preview(&self, data: &GameData) -> InteractionPreview {
        if self.phase != GamePhase::Playing {
            return InteractionPreview::Finished;
        }

        if let Some(active_toy) = self.active_toy() {
            if active_toy.is_repair_part() {
                if self.is_near_repair_bench(data) {
                    // Warn before the placement rather than after the failed
                    // repair: the part already waiting belongs to another toy.
                    if !self.carried_part_matches_bench(data, active_toy) {
                        return InteractionPreview::RepairMismatch;
                    }
                    if self.repair_bench_has_room(data) {
                        return InteractionPreview::PlaceOnRepairBench;
                    }
                    return InteractionPreview::RepairBenchFull;
                }
                if self.targeted_display_slot(data).is_some() || self.is_near_display(data) {
                    return InteractionPreview::NeedsRepair;
                }
                return InteractionPreview::PutDown;
            }
            if let Some(target) = self.targeted_empty_display_slot(data) {
                let _display = &data.displays[target.display_index];
                return InteractionPreview::PlaceOnShelf;
            }
            if let Some(target) = self.targeted_display_slot(data) {
                let display = &data.displays[target.display_index];
                if self.display_is_full(display, data.config.room_width) {
                    return InteractionPreview::ShelfFull;
                }
                return InteractionPreview::LookAtEmptySlot;
            }
            if self.is_near_display(data) {
                return InteractionPreview::LookAtEmptySlot;
            }
            return InteractionPreview::PutDown;
        }

        if self.is_near_repair_bench(data) {
            if let Some(toy_name) = self.benched_repair_name(data) {
                return InteractionPreview::RepairReady { toy_name };
            }
        }

        if let Some(target) = self.targeted_display_slot(data) {
            let display = &data.displays[target.display_index];
            if let Some(toy) =
                self.toy_at_display_slot(display, target.slot_index, data.config.room_width)
            {
                if self.player.carried_toy_ids.len() >= self.carry_limit(&data.config) {
                    return InteractionPreview::InventoryFull;
                }
                return InteractionPreview::Pickup {
                    toy_name: toy.name.clone(),
                };
            }
        }

        if self.player.carried_toy_ids.len() >= self.carry_limit(&data.config) {
            return InteractionPreview::InventoryFull;
        }

        if let Some(toy_index) = self.targeted_loose_toy_index(&data.config) {
            return InteractionPreview::Pickup {
                toy_name: self.toys[toy_index].name.clone(),
            };
        }

        if self.is_near_repair_bench(data) {
            if self.repair_bench_is_full(data) {
                return InteractionPreview::RepairMismatch;
            }
            // Lower priority than picking something up, so standing at a bench
            // never hides a nearby toy's prompt.
            if let Some((toy_name, missing_part)) = self.lone_benched_part(data) {
                return InteractionPreview::AwaitingRepairMatch {
                    toy_name,
                    missing_part,
                };
            }
        }

        InteractionPreview::NothingNearby
    }

    pub fn placed_toys_for_display<'a>(
        &'a self,
        display_id: &'a str,
    ) -> impl Iterator<Item = &'a ToyState> + 'a {
        self.toys.iter().filter(move |toy| {
            toy.placed_display_id
                .as_deref()
                .is_some_and(|placed_id| placed_id == display_id)
        })
    }

    fn drop_active_as_interaction(&mut self, data: &GameData) -> InteractionResult {
        match self.drop_active(data) {
            Some(toy_name) => InteractionResult::Dropped { toy_name },
            None => InteractionResult::NothingNearby,
        }
    }

    pub fn targeted_empty_display_slot(&self, data: &GameData) -> Option<DisplaySlotTarget> {
        let target = self.targeted_display_slot(data)?;
        let display = &data.displays[target.display_index];
        self.toy_index_at_display_slot(display, target.slot_index, data.config.room_width)
            .is_none()
            .then_some(target)
    }

    pub fn targeted_display_slot(&self, data: &GameData) -> Option<DisplaySlotTarget> {
        let player = self.player.position.to_vec2();
        let forward = vec2(self.player.yaw.cos(), self.player.yaw.sin()).normalize_or_zero();
        if forward.length_squared() <= f32::EPSILON {
            return None;
        }

        data.displays
            .iter()
            .enumerate()
            .filter(|(_, display)| self.display_is_near_player(display, &data.config))
            .flat_map(|(display_index, display)| {
                (0..display.capacity).filter_map(move |slot_index| {
                    let slot = display_slot_position(display, slot_index, data.config.room_width)
                        .to_vec2();
                    let to_slot = slot - player;
                    let distance = to_slot.length();
                    let direction = if distance > 0.05 {
                        to_slot / distance
                    } else {
                        forward
                    };
                    let dot = direction.dot(forward);
                    if dot < SLOT_LOOK_MIN_DOT {
                        return None;
                    }

                    let lateral = (to_slot.x * forward.y - to_slot.y * forward.x).abs();
                    if lateral > SLOT_LOOK_MAX_LATERAL && distance > 0.55 {
                        return None;
                    }

                    let score = lateral * 3.6 + (1.0 - dot) * 1.25 + distance * 0.04;
                    Some((
                        DisplaySlotTarget {
                            display_index,
                            slot_index,
                        },
                        score,
                    ))
                })
            })
            .min_by(|(_, left), (_, right)| left.total_cmp(right))
            .map(|(target, _)| target)
    }

    fn toy_at_display_slot(
        &self,
        display: &DisplayDef,
        slot_index: usize,
        room_width: f32,
    ) -> Option<&ToyState> {
        self.toy_index_at_display_slot(display, slot_index, room_width)
            .map(|toy_index| &self.toys[toy_index])
    }

    pub(super) fn pick_up_toy(&mut self, toy_index: usize) -> InteractionResult {
        let toy_id = self.toys[toy_index].id.clone();
        let toy_name = self.toys[toy_index].name.clone();
        let already_carried = self.player.carried_toy_ids.iter().any(|id| id == &toy_id);
        if !already_carried && self.player.carried_toy_ids.len() >= SINGLE_CARRY_LIMIT {
            return InteractionResult::InventoryFull;
        }

        self.toys[toy_index].is_held = true;
        self.toys[toy_index].placed_display_id = None;
        self.toys[toy_index].placed_slot_index = None;
        self.toys[toy_index].bench_slot_index = None;
        self.toys[toy_index].bench_id = None;
        self.toys[toy_index].wrong_marker_seconds = 0.0;
        self.spatial.sync_toy(toy_index, &self.toys[toy_index]);
        if !self.player.carried_toy_ids.iter().any(|id| id == &toy_id) {
            self.player.carried_toy_ids.push(toy_id);
        }
        self.player.active_carry_index = self.player.carried_toy_ids.len().saturating_sub(1);

        InteractionResult::PickedUp { toy_name }
    }

    pub(super) fn place_active_toy(
        &mut self,
        display_index: usize,
        slot_index: usize,
        data: &GameData,
    ) -> InteractionResult {
        let Some(display) = data.displays.get(display_index) else {
            return InteractionResult::NothingNearby;
        };
        if slot_index >= display.capacity {
            return InteractionResult::ShelfSlotUnavailable;
        }
        if self
            .toy_index_at_display_slot(display, slot_index, data.config.room_width)
            .is_some()
        {
            return if self.display_is_full(display, data.config.room_width) {
                InteractionResult::ShelfFull
            } else {
                InteractionResult::ShelfSlotUnavailable
            };
        }

        let toy_id = match self.active_toy() {
            Some(toy) => toy.id.clone(),
            None => return InteractionResult::NothingNearby,
        };
        let toy_index = match self.toys.iter().position(|toy| toy.id == toy_id) {
            Some(index) => index,
            None => return InteractionResult::NothingNearby,
        };
        let toy_name = self.toys[toy_index].name.clone();
        if self.toys[toy_index].is_repair_part() {
            return InteractionResult::NeedsRepair { toy_name };
        }
        let previous_completed_count = self.completed_display_count();

        let is_wrong_display = !toy_matches_display(&self.toys[toy_index], display);
        if is_wrong_display {
            self.player.mistakes += 1;
            self.player.elapsed_seconds += data.config.mistake_penalty_seconds;
        }

        self.toys[toy_index].is_held = false;
        self.toys[toy_index].placed_display_id = Some(display.id.clone());
        self.toys[toy_index].placed_slot_index = Some(slot_index);
        self.toys[toy_index].bench_slot_index = None;
        self.toys[toy_index].bench_id = None;
        self.toys[toy_index].wrong_marker_seconds = if is_wrong_display {
            Self::WRONG_MARKER_SECONDS
        } else {
            0.0
        };
        self.toys[toy_index].position =
            display_slot_position(display, slot_index, data.config.room_width);
        self.spatial.sync_toy(toy_index, &self.toys[toy_index]);
        self.player.carried_toy_ids.retain(|id| id != &toy_id);
        self.normalize_active_carry();

        let was_complete = self.is_display_complete(&display.id);
        self.refresh_display_completion(data);
        let completed_display = if !was_complete && self.is_display_complete(&display.id) {
            Some(display.name.clone())
        } else {
            None
        };
        let available_tools = self.newly_available_upgrades(data, previous_completed_count);
        let finished = self
            .displays
            .iter()
            .all(|display_state| display_state.is_complete);
        if finished {
            self.phase = GamePhase::Finished;
        }

        InteractionResult::Placed {
            toy_name,
            display_name: display.name.clone(),
            was_wrong: is_wrong_display,
            completed_display,
            available_tools,
            finished,
        }
    }

    pub(super) fn repair_display_slots(&mut self, data: &GameData) {
        let player_position = self.player.position;
        for toy in &mut self.toys {
            if toy.is_held {
                toy.placed_display_id = None;
                toy.placed_slot_index = None;
                toy.bench_slot_index = None;
                toy.bench_id = None;
                continue;
            }

            let Some(display_id) = toy.placed_display_id.as_deref() else {
                if toy.bench_slot_index.is_none() {
                    toy.placed_slot_index = None;
                }
                continue;
            };
            toy.bench_slot_index = None;
            toy.bench_id = None;
            if data.display_by_id(display_id).is_none() {
                toy.placed_display_id = None;
                toy.placed_slot_index = None;
            }
        }

        for display in &data.displays {
            let mut used_slots = vec![false; display.capacity];

            for toy in &mut self.toys {
                if toy.placed_display_id.as_deref() != Some(display.id.as_str()) {
                    continue;
                }

                match toy.placed_slot_index {
                    Some(slot_index)
                        if slot_index < display.capacity && !used_slots[slot_index] =>
                    {
                        used_slots[slot_index] = true;
                        toy.position =
                            display_slot_position(display, slot_index, data.config.room_width);
                    }
                    _ => {
                        toy.placed_slot_index = None;
                    }
                }
            }

            for toy in &mut self.toys {
                if toy.placed_display_id.as_deref() != Some(display.id.as_str())
                    || toy.placed_slot_index.is_some()
                {
                    continue;
                }

                let intended_slot = toy.slot_number.saturating_sub(1);
                let slot_index = if intended_slot < display.capacity && !used_slots[intended_slot] {
                    Some(intended_slot)
                } else {
                    used_slots.iter().position(|used| !*used)
                };

                if let Some(slot_index) = slot_index {
                    used_slots[slot_index] = true;
                    toy.placed_slot_index = Some(slot_index);
                    toy.position =
                        display_slot_position(display, slot_index, data.config.room_width);
                } else {
                    toy.placed_display_id = None;
                    toy.placed_slot_index = None;
                    toy.bench_slot_index = None;
                    toy.bench_id = None;
                    toy.wrong_marker_seconds = 0.0;
                    toy.position = player_position;
                }
            }
        }
    }

    fn pick_up_placed_toy(&mut self, toy_index: usize, data: &GameData) -> InteractionResult {
        let result = self.pick_up_toy(toy_index);
        self.refresh_display_completion(data);
        result
    }

    fn toy_index_at_display_slot(
        &self,
        display: &DisplayDef,
        slot_index: usize,
        room_width: f32,
    ) -> Option<usize> {
        let slot = display_slot_position(display, slot_index, room_width).to_vec2();
        self.spatial
            .indices_near(slot, SLOT_OCCUPANCY_RADIUS)
            .into_iter()
            .find(|&toy_index| {
                let toy = &self.toys[toy_index];
                toy.placed_display_id.as_deref() == Some(display.id.as_str())
                    && toy.placed_slot_index == Some(slot_index)
            })
    }

    fn targeted_loose_toy_index(&self, config: &GameConfig) -> Option<usize> {
        let player_2d = self.player.position.to_vec2();
        let eye = vec3(player_2d.x, PLAYER_EYE_HEIGHT, player_2d.y);
        let forward = self.look_forward_3d();
        if forward.length_squared() <= f32::EPSILON {
            return None;
        }

        self.spatial
            .indices_near(player_2d, config.interaction_radius)
            .into_iter()
            .map(|index| (index, &self.toys[index]))
            .filter(|(_, toy)| {
                !toy.is_held && toy.placed_display_id.is_none() && !toy.is_consumed_repair_part()
            })
            .filter_map(|(index, toy)| {
                let horizontal_distance = toy.position.to_vec2().distance(player_2d);
                if horizontal_distance > config.interaction_radius {
                    return None;
                }

                let center = loose_toy_aim_center(index, toy);
                let to_toy = center - eye;
                let along_ray = to_toy.dot(forward);
                if along_ray <= 0.10 {
                    return None;
                }

                let lateral = (to_toy - forward * along_ray).length();
                if lateral > LOOSE_TOY_LOOK_RADIUS {
                    return None;
                }

                let score = lateral * 5.0 + along_ray * 0.08 + horizontal_distance * 0.05;
                Some((index, score))
            })
            .min_by(|(_, left), (_, right)| left.total_cmp(right))
            .map(|(index, _)| index)
    }

    fn is_near_display(&self, data: &GameData) -> bool {
        data.displays
            .iter()
            .any(|display| self.display_is_near_player(display, &data.config))
    }

    fn display_is_full(&self, display: &DisplayDef, room_width: f32) -> bool {
        (0..display.capacity).all(|slot_index| {
            self.toy_index_at_display_slot(display, slot_index, room_width)
                .is_some()
        })
    }

    fn display_is_near_player(&self, display: &DisplayDef, config: &GameConfig) -> bool {
        let player = self.player.position.to_vec2();
        let nearest_point = vec2(
            player.x.clamp(display.x, display.x + display.w),
            player.y.clamp(display.y, display.y + display.h),
        );
        let max_distance_sq = config.interaction_radius * config.interaction_radius;
        nearest_point.distance_squared(player) <= max_distance_sq
    }

    fn look_forward_3d(&self) -> Vec3 {
        vec3(
            self.player.yaw.cos() * self.player.pitch.cos(),
            self.player.pitch.sin(),
            self.player.yaw.sin() * self.player.pitch.cos(),
        )
        .normalize_or_zero()
    }
}

fn loose_toy_aim_center(index: usize, toy: &ToyState) -> Vec3 {
    let layer = (index % 7) as f32;
    let height = if toy.bench_slot_index.is_some() {
        0.88
    } else {
        0.32 + layer * 0.020 + toy.spawn_pose.floor_lift
    };
    vec3(toy.position.x, height, toy.position.y)
}
