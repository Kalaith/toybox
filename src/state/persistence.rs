//! Save payload, load-time repair, and version migration.

use super::{
    DisplayState, GamePhase, GameSession, PlayerState, RepairPartKind, ShiftMode, ToySpatialGrid,
    ToyState,
};
use crate::data::GameData;
use crate::toys::spawn_pose_for_toy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub player: PlayerState,
    pub toys: Vec<ToyState>,
    pub displays: Vec<DisplayState>,
    pub unlocked_upgrade_ids: Vec<String>,
    pub phase: GamePhase,
    /// Saves written before relaxed mode existed carry no mode; they were all
    /// played against the clock, which is what `ShiftMode::default()` is.
    #[serde(default)]
    pub shift_mode: ShiftMode,
}

impl GameSession {
    pub fn from_save(save: SaveData, data: &GameData) -> Self {
        let config = &data.config;
        let mut session = Self {
            player: save.player,
            toys: save.toys,
            displays: save.displays,
            unlocked_upgrade_ids: save.unlocked_upgrade_ids,
            phase: save.phase,
            shift_mode: save.shift_mode,
            spatial: ToySpatialGrid::new(
                config.room_width,
                config.room_height,
                config.spatial_cell_size,
            ),
        };
        session.repair_after_load(data);
        session
    }

    pub fn to_save(&self, version: &str) -> SaveData {
        SaveData {
            version: version.to_owned(),
            player: self.player.clone(),
            toys: self.toys.clone(),
            displays: self.displays.clone(),
            unlocked_upgrade_ids: self.unlocked_upgrade_ids.clone(),
            phase: self.phase,
            shift_mode: self.shift_mode,
        }
    }

    fn repair_after_load(&mut self, data: &GameData) {
        for display in &data.displays {
            if !self.displays.iter().any(|state| state.id == display.id) {
                self.displays.push(DisplayState {
                    id: display.id.clone(),
                    placed_toy_ids: Vec::new(),
                    is_complete: false,
                });
            }
        }

        self.player.carried_toy_ids.retain(|toy_id| {
            self.toys
                .iter()
                .any(|toy| &toy.id == toy_id && !toy.is_consumed_repair_part())
        });

        // `carried_toy_ids` is the authority on what the player holds, but
        // `is_held` is what rendering and pickup targeting read. A save where
        // the two disagree leaves a carried toy drawn on the floor, or a held
        // toy that `targeted_loose_toy_index` skips forever.
        for toy in &mut self.toys {
            let carried = self.player.carried_toy_ids.contains(&toy.id);
            if toy.is_held == carried {
                continue;
            }
            toy.is_held = carried;
            if carried {
                toy.placed_display_id = None;
                toy.placed_slot_index = None;
                toy.bench_slot_index = None;
                toy.bench_id = None;
            }
        }
        for (toy_index, toy) in self.toys.iter_mut().enumerate() {
            if toy.spawn_pose.is_uninitialized() {
                toy.spawn_pose = spawn_pose_for_toy(
                    toy_index,
                    toy.color_index,
                    toy.slot_number.saturating_sub(1),
                );
            }
        }
        if !self.player.mistake_guards_initialized {
            self.player.mistake_guards_remaining = self
                .forgiven_mistakes(data)
                .saturating_sub(self.player.mistakes);
            self.player.mistake_guards_initialized = true;
        }
        self.repair_display_slots(data);
        self.repair_bench_slots(data);
        self.normalize_active_carry(self.carry_limit(data));
        self.repair_player_view();
        self.refresh_display_completion(data);
        self.spatial.rebuild(&self.toys);
    }
}

pub fn migrate_save_value(
    detected_version: Option<String>,
    value: Value,
    data: &GameData,
) -> Result<SaveData, String> {
    let payload = value.get("data").cloned().unwrap_or(value);

    if let Ok(mut current) = serde_json::from_value::<SaveData>(payload) {
        // A save from an older store stocks the wrong toy count entirely
        // (e.g. the 100-toy prototype); restock fresh instead of loading a
        // near-empty shop.
        let live_toys = current
            .toys
            .iter()
            .filter(|toy| {
                !toy.is_consumed_repair_part()
                    && toy.repair_part_kind() != Some(RepairPartKind::Head)
            })
            .count();
        if live_toys != data.config.toy_count {
            eprintln!(
                "Save stocks {} toys but the store now holds {}; restocking fresh",
                live_toys, data.config.toy_count
            );
            return Ok(GameSession::new(data).to_save(&data.config.version));
        }
        current.version = data.config.version.clone();
        return Ok(current);
    }

    eprintln!(
        "Unsupported save format {:?}; starting a clean Toybox session",
        detected_version
    );
    Ok(GameSession::new(data).to_save(&data.config.version))
}
