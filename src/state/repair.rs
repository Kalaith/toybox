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
        self.nearest_bench(data).is_some()
    }

    /// The closest bench whose interaction radius contains the player.
    fn nearest_bench<'a>(&self, data: &'a GameData) -> Option<&'a BenchDef> {
        let player = self.player.position.to_vec2();
        data.layout
            .benches
            .iter()
            .map(|bench| (bench, player.distance_squared(bench_point(bench).to_vec2())))
            .filter(|(bench, distance_sq)| *distance_sq <= bench.radius * bench.radius)
            .min_by(|(_, left), (_, right)| left.total_cmp(right))
            .map(|(bench, _)| bench)
    }

    pub(super) fn repair_bench_has_room(&self, data: &GameData) -> bool {
        self.nearest_bench(data)
            .is_some_and(|bench| self.next_bench_slot(bench).is_some())
    }

    pub(super) fn repair_bench_is_full(&self, data: &GameData) -> bool {
        self.nearest_bench(data)
            .is_some_and(|bench| self.benched_toy_indices(bench).len() >= bench.capacity)
    }

    pub(super) fn benched_repair_name(&self, data: &GameData) -> Option<String> {
        let bench = self.nearest_bench(data)?;
        let part_indices = self.benched_repair_part_indices(bench);
        if part_indices.len() != bench.capacity {
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
        let Some(bench) = self.nearest_bench(data) else {
            return InteractionResult::NothingNearby;
        };
        let Some(slot_index) = self.next_bench_slot(bench) else {
            return InteractionResult::RepairBenchFull;
        };
        let Some(active_index) = self.toys.iter().position(|toy| toy.id == active_id) else {
            return InteractionResult::NothingNearby;
        };

        self.toys[active_index].is_held = false;
        self.toys[active_index].placed_display_id = None;
        self.toys[active_index].placed_slot_index = None;
        self.toys[active_index].bench_slot_index = Some(slot_index);
        self.toys[active_index].bench_id = Some(bench.id.clone());
        self.toys[active_index].wrong_marker_seconds = 0.0;
        self.toys[active_index].position = repair_bench_slot_position(bench, slot_index);
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
        let Some(bench) = self.nearest_bench(data) else {
            return InteractionResult::NothingNearby;
        };
        let consumed_rest_position = repair_bench_slot_position(bench, 1);
        let part_indices = self.benched_repair_part_indices(bench);
        if part_indices.len() != bench.capacity {
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
            self.toys[index].bench_id = None;
            self.toys[index].wrong_marker_seconds = 0.0;
            self.toys[index].position = consumed_rest_position;
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
        self.toys[survivor_index].bench_id = None;
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
        for toy in &mut self.toys {
            // Legacy saves predate bench ids: adopt them onto the primary bench.
            if toy.bench_slot_index.is_some() && toy.bench_id.is_none() {
                toy.bench_id = Some(data.primary_bench().id.clone());
            }
            if toy.bench_slot_index.is_none()
                || toy.bench_id.as_deref().is_some_and(|bench_id| {
                    !data.layout.benches.iter().any(|bench| bench.id == bench_id)
                })
            {
                toy.bench_slot_index = None;
                toy.bench_id = None;
            }
        }

        for bench in &data.layout.benches {
            let mut used_slots = vec![false; bench.capacity];
            for toy in &mut self.toys {
                if toy.bench_id.as_deref() != Some(bench.id.as_str()) {
                    continue;
                }
                let Some(slot_index) = toy.bench_slot_index else {
                    continue;
                };

                if toy.is_held
                    || !toy.is_repair_part()
                    || slot_index >= bench.capacity
                    || used_slots[slot_index]
                {
                    toy.bench_slot_index = None;
                    toy.bench_id = None;
                    continue;
                }

                used_slots[slot_index] = true;
                toy.placed_display_id = None;
                toy.placed_slot_index = None;
                toy.position = repair_bench_slot_position(bench, slot_index);
            }
        }
    }

    fn next_bench_slot(&self, bench: &BenchDef) -> Option<usize> {
        let benched = self.benched_toy_indices(bench);
        (0..bench.capacity).find(|slot_index| {
            !benched
                .iter()
                .any(|&toy_index| self.toys[toy_index].bench_slot_index == Some(*slot_index))
        })
    }

    fn benched_toy_indices(&self, bench: &BenchDef) -> Vec<usize> {
        self.spatial
            .indices_near(bench_point(bench).to_vec2(), bench.radius)
            .into_iter()
            .filter(|&toy_index| {
                let toy = &self.toys[toy_index];
                toy.bench_slot_index.is_some() && toy.bench_id.as_deref() == Some(bench.id.as_str())
            })
            .collect()
    }

    fn benched_repair_part_indices(&self, bench: &BenchDef) -> Vec<usize> {
        let mut parts: Vec<(usize, usize)> = self
            .benched_toy_indices(bench)
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
