use super::{GameSession, InteractionResult, RepairPartKind, RepairState, ToyState, WorldPoint};
use crate::data::{GameData, ToyCategory};

const FIRST_REPAIR_ID: &str = "robot_antenna_001";
const REPAIR_BENCH_POSITION: WorldPoint = WorldPoint { x: 8.8, y: 10.7 };
const REPAIR_BENCH_RADIUS: f32 = 1.45;

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

    pub fn carried_repair_part_count_for_active(&self) -> usize {
        let Some(active_toy) = self.active_toy() else {
            return 0;
        };
        let Some(repair_id) = active_toy.repair_id() else {
            return 0;
        };
        self.carried_repair_part_ids(repair_id).len()
    }

    pub(super) fn repair_active_toy(&mut self) -> InteractionResult {
        let Some(active_toy) = self.active_toy() else {
            return InteractionResult::NothingNearby;
        };
        let Some(repair_id) = active_toy.repair_id().map(str::to_owned) else {
            return InteractionResult::NothingNearby;
        };
        let active_name = active_toy.name.clone();
        let carried_part_ids = self.carried_repair_part_ids(&repair_id);
        if carried_part_ids.len() < 2 {
            return InteractionResult::NeedsRepairParts {
                toy_name: active_name,
            };
        }

        let Some(survivor_id) = self.repair_survivor_id(&carried_part_ids) else {
            return InteractionResult::NeedsRepairParts {
                toy_name: active_name,
            };
        };
        let Some(survivor_index) = self.toys.iter().position(|toy| toy.id == survivor_id) else {
            return InteractionResult::NothingNearby;
        };

        let repaired_name = self.toys[survivor_index]
            .repaired_name()
            .unwrap_or(&self.toys[survivor_index].name)
            .to_owned();

        for consumed_id in carried_part_ids
            .iter()
            .filter(|toy_id| toy_id.as_str() != survivor_id)
        {
            if let Some(index) = self.toys.iter().position(|toy| &toy.id == consumed_id) {
                self.toys[index].is_held = false;
                self.toys[index].placed_display_id = None;
                self.toys[index].placed_slot_index = None;
                self.toys[index].wrong_marker_seconds = 0.0;
                self.toys[index].position = REPAIR_BENCH_POSITION;
                self.toys[index].repair_state = RepairState::ConsumedPart {
                    repair_id: repair_id.clone(),
                };
            }
        }

        self.toys[survivor_index].name = repaired_name.clone();
        self.toys[survivor_index].is_held = true;
        self.toys[survivor_index].placed_display_id = None;
        self.toys[survivor_index].placed_slot_index = None;
        self.toys[survivor_index].wrong_marker_seconds = 0.0;
        self.toys[survivor_index].position = self.player.position;
        self.toys[survivor_index].repair_state = RepairState::Whole;

        self.player.carried_toy_ids.retain(|toy_id| {
            toy_id == &survivor_id || !carried_part_ids.iter().any(|part_id| part_id == toy_id)
        });
        if !self
            .player
            .carried_toy_ids
            .iter()
            .any(|toy_id| toy_id == &survivor_id)
        {
            self.player.carried_toy_ids.push(survivor_id.clone());
        }
        self.player.active_carry_index = self
            .player
            .carried_toy_ids
            .iter()
            .position(|toy_id| toy_id == &survivor_id)
            .unwrap_or(0);

        InteractionResult::Repaired {
            toy_name: repaired_name,
        }
    }

    fn carried_repair_part_ids(&self, repair_id: &str) -> Vec<String> {
        self.player
            .carried_toy_ids
            .iter()
            .filter_map(|toy_id| {
                let toy = self.toys.iter().find(|candidate| &candidate.id == toy_id)?;
                (toy.is_repair_part() && toy.repair_id() == Some(repair_id))
                    .then_some(toy.id.clone())
            })
            .collect()
    }

    fn repair_survivor_id(&self, carried_part_ids: &[String]) -> Option<String> {
        carried_part_ids
            .iter()
            .find(|toy_id| {
                self.toys
                    .iter()
                    .find(|toy| &toy.id == *toy_id)
                    .and_then(ToyState::repair_part_kind)
                    == Some(RepairPartKind::Body)
            })
            .cloned()
            .or_else(|| carried_part_ids.first().cloned())
    }
}

pub fn repair_bench_position() -> WorldPoint {
    REPAIR_BENCH_POSITION
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
