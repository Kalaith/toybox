use super::{GameSession, InteractionResult, RepairPartKind, RepairState, ToyState, WorldPoint};
use crate::data::{BenchDef, GameData, ToyCategory};

const FIRST_REPAIR_ID: &str = "robot_antenna_001";

impl ToyState {
    pub fn is_repair_part(&self) -> bool {
        matches!(self.repair_state, RepairState::BrokenPart { .. })
    }

    pub fn is_consumed_repair_part(&self) -> bool {
        matches!(self.repair_state, RepairState::ConsumedPart { .. })
    }

    pub fn repair_part_kind(&self) -> Option<RepairPartKind> {
        match self.repair_state {
            RepairState::BrokenPart { part, .. } => Some(part),
            _ => None,
        }
    }

    fn repair_id(&self) -> Option<&str> {
        match &self.repair_state {
            RepairState::BrokenPart { repair_id, .. } => Some(repair_id),
            RepairState::ConsumedPart { repair_id } => Some(repair_id),
            RepairState::Whole => None,
        }
    }

    fn repaired_name(&self) -> Option<&str> {
        match &self.repair_state {
            RepairState::BrokenPart { repaired_name, .. } => Some(repaired_name),
            _ => None,
        }
    }
}

impl GameSession {
    pub fn is_near_repair_bench(&self, data: &GameData) -> bool {
        let bench = data.primary_bench();
        self.player
            .position
            .to_vec2()
            .distance_squared(bench_point(bench).to_vec2())
            <= bench.radius * bench.radius
    }

    pub(super) fn repair_bench_has_room(&self, data: &GameData) -> bool {
        self.next_repair_bench_slot(data).is_some()
    }

    pub(super) fn repair_bench_is_full(&self, data: &GameData) -> bool {
        self.benched_toy_indices(data).len() >= data.primary_bench().capacity
    }

    pub(super) fn benched_repair_name(&self, data: &GameData) -> Option<String> {
        let part_indices = self.benched_repair_part_indices(data);
        if part_indices.len() != data.primary_bench().capacity {
            return None;
        }

        let first_part = &self.toys[part_indices[0]];
        let repair_id = first_part.repair_id()?;
        let matching_parts = part_indices
            .iter()
            .all(|&toy_index| self.toys[toy_index].repair_id() == Some(repair_id));

        matching_parts.then(|| {
            first_part
                .repaired_name()
                .unwrap_or(&first_part.name)
                .to_owned()
        })
    }

    pub(super) fn place_active_on_repair_bench(&mut self, data: &GameData) -> InteractionResult {
        let Some(active_toy) = self.active_toy() else {
            return InteractionResult::NothingNearby;
        };
        let active_id = active_toy.id.clone();
        let active_name = active_toy.name.clone();
        if !active_toy.is_repair_part() {
            return InteractionResult::NothingNearby;
        };
        let Some(slot_index) = self.next_repair_bench_slot(data) else {
            return InteractionResult::RepairBenchFull;
        };
        let Some(active_index) = self.toys.iter().position(|toy| toy.id == active_id) else {
            return InteractionResult::NothingNearby;
        };

        self.toys[active_index].is_held = false;
        self.toys[active_index].placed_display_id = None;
        self.toys[active_index].placed_slot_index = None;
        self.toys[active_index].bench_slot_index = Some(slot_index);
        self.toys[active_index].wrong_marker_seconds = 0.0;
        self.toys[active_index].position =
            repair_bench_slot_position(data.primary_bench(), slot_index);
        self.spatial
            .sync_toy(active_index, &self.toys[active_index]);

        self.player
            .carried_toy_ids
            .retain(|toy_id| toy_id != &active_id);
        self.normalize_active_carry();

        InteractionResult::PlacedOnRepairBench {
            toy_name: active_name,
        }
    }

    pub(super) fn repair_benched_toys(&mut self, data: &GameData) -> InteractionResult {
        let part_indices = self.benched_repair_part_indices(data);
        if part_indices.len() != data.primary_bench().capacity {
            return InteractionResult::NeedsRepairParts {
                toy_name: "repair".to_owned(),
            };
        }
        let Some(repaired_name) = self.benched_repair_name(data) else {
            return InteractionResult::RepairMismatch;
        };
        let survivor_index = part_indices
            .iter()
            .copied()
            .find(|&toy_index| {
                self.toys[toy_index].repair_part_kind() == Some(RepairPartKind::Body)
            })
            .unwrap_or(part_indices[0]);
        let Some(repair_id) = self.toys[survivor_index].repair_id().map(str::to_owned) else {
            return InteractionResult::RepairMismatch;
        };

        for &index in part_indices
            .iter()
            .filter(|&&toy_index| toy_index != survivor_index)
        {
            self.toys[index].is_held = false;
            self.toys[index].placed_display_id = None;
            self.toys[index].placed_slot_index = None;
            self.toys[index].bench_slot_index = None;
            self.toys[index].wrong_marker_seconds = 0.0;
            self.toys[index].position = repair_bench_slot_position(data.primary_bench(), 1);
            self.toys[index].repair_state = RepairState::ConsumedPart {
                repair_id: repair_id.clone(),
            };
            self.spatial.sync_toy(index, &self.toys[index]);
        }

        self.toys[survivor_index].name = repaired_name.clone();
        self.toys[survivor_index].is_held = true;
        self.toys[survivor_index].placed_display_id = None;
        self.toys[survivor_index].placed_slot_index = None;
        self.toys[survivor_index].bench_slot_index = None;
        self.toys[survivor_index].wrong_marker_seconds = 0.0;
        self.toys[survivor_index].position = self.player.position;
        self.toys[survivor_index].repair_state = RepairState::Whole;
        self.spatial
            .sync_toy(survivor_index, &self.toys[survivor_index]);

        self.player.carried_toy_ids.clear();
        self.player
            .carried_toy_ids
            .push(self.toys[survivor_index].id.clone());
        self.player.active_carry_index = 0;

        InteractionResult::Repaired {
            toy_name: repaired_name,
        }
    }

    pub(super) fn repair_bench_slots(&mut self, data: &GameData) {
        let bench = data.primary_bench();
        let mut used_slots = vec![false; bench.capacity];
        for toy in &mut self.toys {
            let Some(slot_index) = toy.bench_slot_index else {
                continue;
            };

            if toy.is_held
                || !toy.is_repair_part()
                || slot_index >= bench.capacity
                || used_slots[slot_index]
            {
                toy.bench_slot_index = None;
                continue;
            }

            used_slots[slot_index] = true;
            toy.placed_display_id = None;
            toy.placed_slot_index = None;
            toy.position = repair_bench_slot_position(bench, slot_index);
        }
    }

    fn next_repair_bench_slot(&self, data: &GameData) -> Option<usize> {
        let benched = self.benched_toy_indices(data);
        (0..data.primary_bench().capacity).find(|slot_index| {
            !benched
                .iter()
                .any(|&toy_index| self.toys[toy_index].bench_slot_index == Some(*slot_index))
        })
    }

    fn benched_toy_indices(&self, data: &GameData) -> Vec<usize> {
        let bench = data.primary_bench();
        self.spatial
            .indices_near(bench_point(bench).to_vec2(), bench.radius)
            .into_iter()
            .filter(|&toy_index| self.toys[toy_index].bench_slot_index.is_some())
            .collect()
    }

    fn benched_repair_part_indices(&self, data: &GameData) -> Vec<usize> {
        let mut parts: Vec<(usize, usize)> = self
            .benched_toy_indices(data)
            .into_iter()
            .filter(|&toy_index| self.toys[toy_index].is_repair_part())
            .filter_map(|toy_index| {
                self.toys[toy_index]
                    .bench_slot_index
                    .map(|slot_index| (slot_index, toy_index))
            })
            .collect();
        parts.sort_by_key(|(slot_index, _)| *slot_index);
        parts.into_iter().map(|(_, toy_index)| toy_index).collect()
    }
}

fn bench_point(bench: &BenchDef) -> WorldPoint {
    WorldPoint {
        x: bench.x,
        y: bench.y,
    }
}

fn repair_bench_slot_position(bench: &BenchDef, slot_index: usize) -> WorldPoint {
    let x_offset = if slot_index == 0 { -0.38 } else { 0.38 };
    WorldPoint {
        x: bench.x + x_offset,
        y: bench.y - 0.08,
    }
}

pub(super) fn split_initial_broken_toys(toys: &mut Vec<ToyState>, data: &GameData) {
    let Some(robot_index) = toys.iter().position(|toy| {
        toy.category == ToyCategory::ActionFigures
            && toy.slot_number == 1
            && toy.theme == "Chrome Bot Wave"
    }) else {
        return;
    };
    if toys[robot_index].is_repair_part() {
        return;
    }

    let repaired_name = toys[robot_index].name.clone();
    let mut head = toys[robot_index].clone();
    head.id = format!("repair_{}_head", FIRST_REPAIR_ID);
    head.name = format!("{repaired_name} Head");
    head.position = safe_part_position(data, 6.35, 9.35);
    head.repair_state = RepairState::BrokenPart {
        repair_id: FIRST_REPAIR_ID.to_owned(),
        part: RepairPartKind::Head,
        repaired_name: repaired_name.clone(),
    };

    toys[robot_index].name = format!("{repaired_name} Body");
    toys[robot_index].repair_state = RepairState::BrokenPart {
        repair_id: FIRST_REPAIR_ID.to_owned(),
        part: RepairPartKind::Body,
        repaired_name,
    };

    toys.push(head);
}

fn safe_part_position(data: &GameData, x: f32, y: f32) -> WorldPoint {
    WorldPoint {
        x: x.clamp(0.8, data.config.room_width - 0.8),
        y: y.clamp(0.8, data.config.room_height - 0.8),
    }
}
