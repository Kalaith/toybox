use super::{GameSession, InteractionResult, RepairPartKind, RepairState, ToyState, WorldPoint};
use crate::data::{GameData, ToyCategory};

const FIRST_REPAIR_ID: &str = "robot_antenna_001";
const REPAIR_BENCH_POSITION: WorldPoint = WorldPoint { x: 8.8, y: 10.7 };
const REPAIR_BENCH_RADIUS: f32 = 1.45;
const REPAIR_BENCH_CAPACITY: usize = 2;

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
    pub fn is_near_repair_bench(&self) -> bool {
        self.player
            .position
            .to_vec2()
            .distance_squared(REPAIR_BENCH_POSITION.to_vec2())
            <= REPAIR_BENCH_RADIUS * REPAIR_BENCH_RADIUS
    }

    pub(super) fn repair_bench_has_room(&self) -> bool {
        self.next_repair_bench_slot().is_some()
    }

    pub(super) fn repair_bench_is_full(&self) -> bool {
        self.toys
            .iter()
            .filter(|toy| toy.bench_slot_index.is_some())
            .count()
            >= REPAIR_BENCH_CAPACITY
    }

    pub(super) fn benched_repair_name(&self) -> Option<String> {
        let part_ids = self.benched_repair_part_ids();
        if part_ids.len() != REPAIR_BENCH_CAPACITY {
            return None;
        }

        let first_part = self.toys.iter().find(|toy| toy.id == part_ids[0])?;
        let repair_id = first_part.repair_id()?;
        let matching_parts = part_ids.iter().all(|toy_id| {
            self.toys
                .iter()
                .find(|toy| &toy.id == toy_id)
                .and_then(ToyState::repair_id)
                == Some(repair_id)
        });

        matching_parts.then(|| {
            first_part
                .repaired_name()
                .unwrap_or(&first_part.name)
                .to_owned()
        })
    }

    pub(super) fn place_active_on_repair_bench(&mut self) -> InteractionResult {
        let Some(active_toy) = self.active_toy() else {
            return InteractionResult::NothingNearby;
        };
        let active_id = active_toy.id.clone();
        let active_name = active_toy.name.clone();
        if !active_toy.is_repair_part() {
            return InteractionResult::NothingNearby;
        };
        let Some(slot_index) = self.next_repair_bench_slot() else {
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
        self.toys[active_index].position = repair_bench_slot_position(slot_index);

        self.player
            .carried_toy_ids
            .retain(|toy_id| toy_id != &active_id);
        self.normalize_active_carry();

        InteractionResult::PlacedOnRepairBench {
            toy_name: active_name,
        }
    }

    pub(super) fn repair_benched_toys(&mut self) -> InteractionResult {
        let part_ids = self.benched_repair_part_ids();
        if part_ids.len() != REPAIR_BENCH_CAPACITY {
            return InteractionResult::NeedsRepairParts {
                toy_name: "repair".to_owned(),
            };
        }
        let Some(repaired_name) = self.benched_repair_name() else {
            return InteractionResult::RepairMismatch;
        };
        let Some(survivor_id) = self.repair_survivor_id(&part_ids) else {
            return InteractionResult::RepairMismatch;
        };
        let Some(survivor_index) = self.toys.iter().position(|toy| toy.id == survivor_id) else {
            return InteractionResult::NothingNearby;
        };
        let Some(repair_id) = self.toys[survivor_index].repair_id().map(str::to_owned) else {
            return InteractionResult::RepairMismatch;
        };

        for consumed_id in part_ids
            .iter()
            .filter(|toy_id| toy_id.as_str() != survivor_id)
        {
            if let Some(index) = self.toys.iter().position(|toy| &toy.id == consumed_id) {
                self.toys[index].is_held = false;
                self.toys[index].placed_display_id = None;
                self.toys[index].placed_slot_index = None;
                self.toys[index].bench_slot_index = None;
                self.toys[index].wrong_marker_seconds = 0.0;
                self.toys[index].position = repair_bench_slot_position(1);
                self.toys[index].repair_state = RepairState::ConsumedPart {
                    repair_id: repair_id.clone(),
                };
            }
        }

        self.toys[survivor_index].name = repaired_name.clone();
        self.toys[survivor_index].is_held = true;
        self.toys[survivor_index].placed_display_id = None;
        self.toys[survivor_index].placed_slot_index = None;
        self.toys[survivor_index].bench_slot_index = None;
        self.toys[survivor_index].wrong_marker_seconds = 0.0;
        self.toys[survivor_index].position = self.player.position;
        self.toys[survivor_index].repair_state = RepairState::Whole;

        self.player.carried_toy_ids.clear();
        self.player.carried_toy_ids.push(survivor_id);
        self.player.active_carry_index = 0;

        InteractionResult::Repaired {
            toy_name: repaired_name,
        }
    }

    pub(super) fn repair_bench_slots(&mut self) {
        let mut used_slots = [false; REPAIR_BENCH_CAPACITY];
        for toy in &mut self.toys {
            let Some(slot_index) = toy.bench_slot_index else {
                continue;
            };

            if toy.is_held
                || !toy.is_repair_part()
                || slot_index >= REPAIR_BENCH_CAPACITY
                || used_slots[slot_index]
            {
                toy.bench_slot_index = None;
                continue;
            }

            used_slots[slot_index] = true;
            toy.placed_display_id = None;
            toy.placed_slot_index = None;
            toy.position = repair_bench_slot_position(slot_index);
        }
    }

    fn next_repair_bench_slot(&self) -> Option<usize> {
        (0..REPAIR_BENCH_CAPACITY).find(|slot_index| {
            !self
                .toys
                .iter()
                .any(|toy| toy.bench_slot_index == Some(*slot_index))
        })
    }

    fn benched_repair_part_ids(&self) -> Vec<String> {
        let mut parts: Vec<(usize, String)> = self
            .toys
            .iter()
            .filter(|toy| toy.is_repair_part())
            .filter_map(|toy| {
                toy.bench_slot_index
                    .map(|slot_index| (slot_index, toy.id.clone()))
            })
            .collect();
        parts.sort_by_key(|(slot_index, _)| *slot_index);
        parts.into_iter().map(|(_, toy_id)| toy_id).collect()
    }

    fn repair_survivor_id(&self, part_ids: &[String]) -> Option<String> {
        part_ids
            .iter()
            .find(|toy_id| {
                self.toys
                    .iter()
                    .find(|toy| &toy.id == *toy_id)
                    .and_then(ToyState::repair_part_kind)
                    == Some(RepairPartKind::Body)
            })
            .cloned()
            .or_else(|| part_ids.first().cloned())
    }
}

pub fn repair_bench_position() -> WorldPoint {
    REPAIR_BENCH_POSITION
}

fn repair_bench_slot_position(slot_index: usize) -> WorldPoint {
    let x_offset = if slot_index == 0 { -0.38 } else { 0.38 };
    WorldPoint {
        x: REPAIR_BENCH_POSITION.x + x_offset,
        y: REPAIR_BENCH_POSITION.y - 0.08,
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
