//! Save payload, load-time repair, and version migration.

use super::{
    DisplayState, GamePhase, GameSession, PlayerState, RepairPartKind, ToySpatialGrid, ToyState,
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
        for (toy_index, toy) in self.toys.iter_mut().enumerate() {
            if toy.spawn_pose.is_uninitialized() {
                toy.spawn_pose = spawn_pose_for_toy(
                    toy_index,
                    toy.color_index,
                    toy.slot_number.saturating_sub(1),
                );
            }
        }
        self.repair_display_slots(data);
        self.repair_bench_slots(data);
        self.normalize_active_carry();
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
