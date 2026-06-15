//! Runtime toy-store state, save data, and save migration helpers.

use crate::data::{DisplayDef, GameConfig, GameData, ToyCategory};
use crate::toys::{spawn_pose_for_toy, toy_name, ToySpawnPose};
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod interactions;
mod repair;

pub use repair::repair_bench_position;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WorldPoint {
    pub x: f32,
    pub y: f32,
}

impl WorldPoint {
    fn from_vec2(position: Vec2) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }

    pub fn to_vec2(self) -> Vec2 {
        vec2(self.x, self.y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub position: WorldPoint,
    #[serde(default = "default_player_yaw")]
    pub yaw: f32,
    #[serde(default)]
    pub pitch: f32,
    pub carried_toy_ids: Vec<String>,
    pub active_carry_index: usize,
    pub mistakes: u32,
    pub elapsed_seconds: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToyState {
    pub id: String,
    pub name: String,
    pub category: ToyCategory,
    pub theme: String,
    pub slot_number: usize,
    pub color_index: usize,
    pub position: WorldPoint,
    #[serde(default)]
    pub spawn_pose: ToySpawnPose,
    pub is_held: bool,
    pub placed_display_id: Option<String>,
    #[serde(default)]
    pub placed_slot_index: Option<usize>,
    #[serde(default)]
    pub bench_slot_index: Option<usize>,
    #[serde(default)]
    pub wrong_marker_seconds: f32,
    #[serde(default)]
    pub repair_state: RepairState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum RepairState {
    Whole,
    BrokenPart {
        repair_id: String,
        part: RepairPartKind,
        repaired_name: String,
    },
    ConsumedPart {
        repair_id: String,
    },
}

impl Default for RepairState {
    fn default() -> Self {
        Self::Whole
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairPartKind {
    Head,
    Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayState {
    pub id: String,
    pub placed_toy_ids: Vec<String>,
    pub is_complete: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Playing,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub player: PlayerState,
    pub toys: Vec<ToyState>,
    pub displays: Vec<DisplayState>,
    pub unlocked_upgrade_ids: Vec<String>,
    pub phase: GamePhase,
}

#[derive(Debug, Clone)]
pub struct GameSession {
    pub player: PlayerState,
    pub toys: Vec<ToyState>,
    pub displays: Vec<DisplayState>,
    pub unlocked_upgrade_ids: Vec<String>,
    pub phase: GamePhase,
}

#[derive(Debug, Clone)]
pub enum InteractionResult {
    PickedUp {
        toy_name: String,
    },
    Dropped {
        toy_name: String,
    },
    Placed {
        toy_name: String,
        display_name: String,
        was_wrong: bool,
        completed_display: Option<String>,
        available_tools: Vec<String>,
        finished: bool,
    },
    PlacedOnRepairBench {
        toy_name: String,
    },
    Repaired {
        toy_name: String,
    },
    NeedsRepair {
        toy_name: String,
    },
    NeedsRepairParts {
        toy_name: String,
    },
    InventoryFull,
    RepairBenchFull,
    RepairMismatch,
    ShelfFull,
    ShelfSlotUnavailable,
    NothingNearby,
}

#[derive(Debug, Clone)]
pub enum InteractionPreview {
    PlaceOnShelf,
    PlaceOnRepairBench,
    RepairReady { toy_name: String },
    RepairBenchFull,
    RepairMismatch,
    NeedsRepair,
    PutDown,
    Pickup { toy_name: String },
    InventoryFull,
    ShelfFull,
    LookAtEmptySlot,
    NothingNearby,
    Finished,
}

#[derive(Debug, Clone)]
pub enum ToolPurchaseResult {
    Purchased {
        tool_name: String,
        remaining_credits: usize,
    },
    AlreadyOwned {
        tool_name: String,
    },
    Locked {
        tool_name: String,
        required_displays: usize,
        completed_displays: usize,
    },
    NeedMoreCredits {
        tool_name: String,
        cost: usize,
        available_credits: usize,
    },
    NoToolsAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplaySlotTarget {
    pub display_index: usize,
    pub slot_index: usize,
}

const TOY_SCANNER_ID: &str = "toy_scanner";
const LEGACY_TAG_LANTERN_ID: &str = "tag_lantern";
const SINGLE_CARRY_LIMIT: usize = 1;

impl GameSession {
    pub const MAX_LOOK_PITCH: f32 = 1.18;
    pub const WRONG_MARKER_SECONDS: f32 = 2.5;

    pub fn new(data: &GameData) -> Self {
        let config = &data.config;
        let toys = build_toys(data);
        let displays = data
            .displays
            .iter()
            .map(|display| DisplayState {
                id: display.id.clone(),
                placed_toy_ids: Vec::new(),
                is_complete: false,
            })
            .collect();

        Self {
            player: PlayerState {
                position: WorldPoint {
                    x: config.room_width * 0.63,
                    y: config.room_height * 0.72,
                },
                yaw: default_player_yaw(),
                pitch: -0.12,
                carried_toy_ids: Vec::new(),
                active_carry_index: 0,
                mistakes: 0,
                elapsed_seconds: 0.0,
            },
            toys,
            displays,
            unlocked_upgrade_ids: Vec::new(),
            phase: GamePhase::Playing,
        }
    }

    pub fn from_save(save: SaveData, data: &GameData) -> Self {
        let mut session = Self {
            player: save.player,
            toys: save.toys,
            displays: save.displays,
            unlocked_upgrade_ids: save.unlocked_upgrade_ids,
            phase: save.phase,
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

    pub fn update_timer(&mut self, dt: f32) {
        if self.phase == GamePhase::Playing {
            self.player.elapsed_seconds += dt;
        }
        for toy in &mut self.toys {
            toy.wrong_marker_seconds = (toy.wrong_marker_seconds - dt).max(0.0);
        }
    }

    pub fn update_player_look(&mut self, yaw_delta: f32, pitch_delta: f32) {
        if self.phase != GamePhase::Playing {
            return;
        }

        self.player.yaw += yaw_delta;
        self.player.pitch =
            (self.player.pitch + pitch_delta).clamp(-Self::MAX_LOOK_PITCH, Self::MAX_LOOK_PITCH);
    }

    pub fn move_player(&mut self, direction: Vec2, config: &GameConfig, dt: f32) {
        if self.phase != GamePhase::Playing || direction.length_squared() == 0.0 {
            return;
        }

        let forward = vec2(self.player.yaw.cos(), self.player.yaw.sin());
        let right = vec2(-forward.y, forward.x);
        let world_direction = right * direction.x + forward * direction.y;
        if world_direction.length_squared() == 0.0 {
            return;
        }

        let next =
            self.player.position.to_vec2() + world_direction.normalize() * config.player_speed * dt;
        let clamped = vec2(
            next.x.clamp(0.45, config.room_width - 0.45),
            next.y.clamp(0.45, config.room_height - 0.45),
        );
        self.player.position = WorldPoint::from_vec2(clamped);
    }

    pub fn carry_limit(&self, _config: &GameConfig) -> usize {
        SINGLE_CARRY_LIMIT
    }

    pub fn has_upgrade(&self, upgrade_id: &str) -> bool {
        let has_exact_match = self
            .unlocked_upgrade_ids
            .iter()
            .any(|existing_id| existing_id == upgrade_id);
        has_exact_match
            || upgrade_id == TOY_SCANNER_ID
                && self
                    .unlocked_upgrade_ids
                    .iter()
                    .any(|existing_id| existing_id == LEGACY_TAG_LANTERN_ID)
    }

    pub fn scanner_enabled(&self) -> bool {
        self.has_upgrade(TOY_SCANNER_ID)
    }

    pub fn available_tool_credits(&self, data: &GameData) -> usize {
        self.completed_display_count()
            .saturating_sub(self.spent_tool_credits(data))
    }

    pub fn next_available_upgrade<'a>(
        &self,
        data: &'a GameData,
    ) -> Option<&'a crate::data::UpgradeDef> {
        let completed_count = self.completed_display_count();
        data.upgrades.iter().find(|upgrade| {
            completed_count >= upgrade.unlock_completed_displays && !self.has_upgrade(&upgrade.id)
        })
    }

    pub fn purchase_tool(&mut self, data: &GameData, upgrade_id: &str) -> ToolPurchaseResult {
        let Some(upgrade) = data
            .upgrades
            .iter()
            .find(|upgrade| upgrade.id == upgrade_id)
        else {
            return ToolPurchaseResult::NoToolsAvailable;
        };
        if self.has_upgrade(&upgrade.id) {
            return ToolPurchaseResult::AlreadyOwned {
                tool_name: upgrade.name.clone(),
            };
        }

        let completed_displays = self.completed_display_count();
        if completed_displays < upgrade.unlock_completed_displays {
            return ToolPurchaseResult::Locked {
                tool_name: upgrade.name.clone(),
                required_displays: upgrade.unlock_completed_displays,
                completed_displays,
            };
        }

        let available_credits = self.available_tool_credits(data);
        if available_credits < upgrade.cost {
            return ToolPurchaseResult::NeedMoreCredits {
                tool_name: upgrade.name.clone(),
                cost: upgrade.cost,
                available_credits,
            };
        }

        self.unlocked_upgrade_ids.push(upgrade.id.clone());
        ToolPurchaseResult::Purchased {
            tool_name: upgrade.name.clone(),
            remaining_credits: self.available_tool_credits(data),
        }
    }

    pub fn active_toy(&self) -> Option<&ToyState> {
        let toy_id = self
            .player
            .carried_toy_ids
            .get(self.player.active_carry_index)?;
        self.toys.iter().find(|toy| &toy.id == toy_id)
    }

    pub fn cycle_carried(&mut self) {
        if self.player.carried_toy_ids.is_empty() {
            self.player.active_carry_index = 0;
        } else {
            self.player.active_carry_index =
                (self.player.active_carry_index + 1) % self.player.carried_toy_ids.len();
        }
    }

    pub fn drop_active(&mut self, data: &GameData) -> Option<String> {
        let toy_id = self.active_toy()?.id.clone();
        let toy_index = self.toys.iter().position(|toy| toy.id == toy_id)?;
        let toy_name = self.toys[toy_index].name.clone();
        let drop_position = self.floor_drop_position(data);

        self.toys[toy_index].is_held = false;
        self.toys[toy_index].placed_display_id = None;
        self.toys[toy_index].placed_slot_index = None;
        self.toys[toy_index].bench_slot_index = None;
        self.toys[toy_index].wrong_marker_seconds = 0.0;
        self.toys[toy_index].position = drop_position;
        self.player.carried_toy_ids.retain(|id| id != &toy_id);
        self.normalize_active_carry();

        Some(toy_name)
    }

    fn floor_drop_position(&self, data: &GameData) -> WorldPoint {
        let config = &data.config;
        let forward = vec2(self.player.yaw.cos(), self.player.yaw.sin()).normalize_or_zero();
        let offset = if forward.length_squared() > f32::EPSILON {
            forward * 0.82
        } else {
            Vec2::ZERO
        };
        let position = self.player.position.to_vec2() + offset;
        let clamped = vec2(
            position.x.clamp(0.65, config.room_width - 0.65),
            position.y.clamp(0.65, config.room_height - 0.65),
        );
        let off_displays = keep_off_displays(clamped, data);

        WorldPoint {
            x: off_displays.x.clamp(0.65, config.room_width - 0.65),
            y: off_displays.y.clamp(0.65, config.room_height - 0.65),
        }
    }

    pub fn completed_display_count(&self) -> usize {
        self.displays
            .iter()
            .filter(|display| display.is_complete)
            .count()
    }

    pub fn total_placed_toys(&self) -> usize {
        self.displays
            .iter()
            .map(|display| display.placed_toy_ids.len())
            .sum()
    }

    pub fn is_display_complete(&self, display_id: &str) -> bool {
        self.displays
            .iter()
            .find(|display| display.id == display_id)
            .map(|display| display.is_complete)
            .unwrap_or(false)
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
        self.repair_bench_slots();
        self.normalize_active_carry();
        self.repair_player_view();
        self.refresh_display_completion(data);
    }

    fn refresh_display_completion(&mut self, data: &GameData) {
        for display_state in &mut self.displays {
            if let Some(display) = data.display_by_id(&display_state.id) {
                display_state.placed_toy_ids = self
                    .toys
                    .iter()
                    .filter(|toy| {
                        toy.placed_display_id
                            .as_deref()
                            .is_some_and(|placed_id| placed_id == display.id)
                    })
                    .map(|toy| toy.id.clone())
                    .collect();
                display_state.is_complete = (0..display.capacity).all(|slot_index| {
                    self.toys.iter().any(|toy| {
                        toy.placed_display_id
                            .as_deref()
                            .is_some_and(|placed_id| placed_id == display.id)
                            && toy.placed_slot_index == Some(slot_index)
                            && toy_matches_display(toy, display)
                    })
                });
            }
        }
    }

    fn newly_available_upgrades(
        &self,
        data: &GameData,
        previous_completed_count: usize,
    ) -> Vec<String> {
        let completed_count = self.completed_display_count();
        data.upgrades
            .iter()
            .filter(|upgrade| {
                previous_completed_count < upgrade.unlock_completed_displays
                    && completed_count >= upgrade.unlock_completed_displays
                    && !self.has_upgrade(&upgrade.id)
            })
            .map(|upgrade| upgrade.name.clone())
            .collect()
    }

    fn spent_tool_credits(&self, data: &GameData) -> usize {
        self.unlocked_upgrade_ids
            .iter()
            .filter_map(|upgrade_id| {
                data.upgrades
                    .iter()
                    .find(|upgrade| &upgrade.id == upgrade_id)
                    .or_else(|| {
                        if upgrade_id == LEGACY_TAG_LANTERN_ID {
                            data.upgrades
                                .iter()
                                .find(|upgrade| upgrade.id == TOY_SCANNER_ID)
                        } else {
                            None
                        }
                    })
            })
            .map(|upgrade| upgrade.cost)
            .sum()
    }

    fn normalize_active_carry(&mut self) {
        if self.player.carried_toy_ids.len() > SINGLE_CARRY_LIMIT {
            let kept_id = self
                .player
                .carried_toy_ids
                .get(self.player.active_carry_index)
                .cloned()
                .or_else(|| self.player.carried_toy_ids.first().cloned());
            self.player.carried_toy_ids.retain(|toy_id| {
                kept_id
                    .as_ref()
                    .is_some_and(|kept_toy_id| toy_id == kept_toy_id)
            });
            for toy in &mut self.toys {
                if toy.is_held && Some(&toy.id) != kept_id.as_ref() {
                    toy.is_held = false;
                    toy.placed_display_id = None;
                    toy.placed_slot_index = None;
                    toy.bench_slot_index = None;
                    toy.wrong_marker_seconds = 0.0;
                    toy.position = self.player.position;
                }
            }
        }

        if self.player.carried_toy_ids.is_empty() {
            self.player.active_carry_index = 0;
        } else if self.player.active_carry_index >= self.player.carried_toy_ids.len() {
            self.player.active_carry_index = self.player.carried_toy_ids.len() - 1;
        }
    }

    fn repair_player_view(&mut self) {
        if !self.player.yaw.is_finite() {
            self.player.yaw = default_player_yaw();
        }
        if !self.player.pitch.is_finite() {
            self.player.pitch = 0.0;
        }
        self.player.pitch = self
            .player
            .pitch
            .clamp(-Self::MAX_LOOK_PITCH, Self::MAX_LOOK_PITCH);
    }
}

pub fn toy_matches_display(toy: &ToyState, display: &DisplayDef) -> bool {
    matches!(toy.repair_state, RepairState::Whole)
        && toy.category == display.category
        && toy.theme == display.theme
}

pub fn format_elapsed_time(seconds: f32) -> String {
    let total_seconds = seconds.max(0.0).floor() as u32;
    let minutes = total_seconds / 60;
    let remaining_seconds = total_seconds % 60;
    format!("{minutes:02}:{remaining_seconds:02}")
}

pub fn migrate_save_value(
    detected_version: Option<String>,
    value: Value,
    data: &GameData,
) -> Result<SaveData, String> {
    let payload = value.get("data").cloned().unwrap_or(value);

    if let Ok(mut current) = serde_json::from_value::<SaveData>(payload) {
        current.version = data.config.version.clone();
        return Ok(current);
    }

    eprintln!(
        "Unsupported save format {:?}; starting a clean Toybox session",
        detected_version
    );
    Ok(GameSession::new(data).to_save(&data.config.version))
}

fn build_toys(data: &GameData) -> Vec<ToyState> {
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
    let (anchor, radius) = mess_pile_anchor(toy_index, display_index, slot_index);
    let angle = toy_index as f32 * 2.399 + display_index as f32 * 0.77 + slot_index as f32 * 0.19;
    let ring_seed = ((toy_index * 37 + display_index * 11 + slot_index * 17) % 100) as f32 / 100.0;
    let spill = if toy_index % 11 == 0 { 1.38 } else { 1.0 };
    let squash = 0.56 + ((toy_index * 7 + slot_index * 5) % 30) as f32 / 100.0;
    let offset = vec2(
        angle.cos() * radius * ring_seed * spill,
        angle.sin() * radius * squash,
    );
    let jitter = vec2(
        (((toy_index * 41) % 23) as f32 - 11.0) * 0.018,
        (((toy_index * 59) % 29) as f32 - 14.0) * 0.016,
    );
    let position = keep_off_displays(anchor + offset + jitter, data);

    WorldPoint {
        x: position.x.clamp(0.8, config.room_width - 0.8),
        y: position.y.clamp(0.8, config.room_height - 0.8),
    }
}

fn mess_pile_anchor(toy_index: usize, display_index: usize, slot_index: usize) -> (Vec2, f32) {
    let pile_slot = (toy_index * 7 + display_index * 3 + slot_index) % 32;
    match pile_slot {
        0..=6 => (vec2(6.25, 5.72), 1.18),
        7..=12 => (vec2(11.55, 5.68), 1.14),
        13..=16 => (vec2(8.95, 3.82), 1.26),
        17..=19 => (vec2(9.12, 8.15), 1.30),
        20..=22 => (vec2(3.78, 3.25), 1.05),
        23..=24 => (vec2(14.10, 3.22), 1.04),
        25..=26 => (vec2(3.52, 7.45), 1.00),
        27..=28 => (vec2(14.35, 7.62), 1.02),
        29 => (vec2(5.35, 9.35), 0.82),
        30 => (vec2(12.65, 9.28), 0.84),
        _ => (vec2(9.05, 10.08), 0.94),
    }
}

fn keep_off_displays(mut position: Vec2, data: &GameData) -> Vec2 {
    for display in &data.displays {
        let margin = 0.18;
        let left = display.x - margin;
        let right = display.x + display.w + margin;
        let top = display.y - margin;
        let bottom = display.y + display.h + margin;
        if position.x < left || position.x > right || position.y < top || position.y > bottom {
            continue;
        }

        let distances = [
            (position.x - left, vec2(left - 0.26, position.y)),
            (right - position.x, vec2(right + 0.26, position.y)),
            (position.y - top, vec2(position.x, top - 0.26)),
            (bottom - position.y, vec2(position.x, bottom + 0.26)),
        ];
        position = distances
            .iter()
            .min_by(|(left_distance, _), (right_distance, _)| {
                left_distance.total_cmp(right_distance)
            })
            .map(|(_, nudged)| *nudged)
            .unwrap_or(position);
    }

    position
}

fn default_player_yaw() -> f32 {
    -std::f32::consts::FRAC_PI_2
}

pub fn display_slot_position(
    display: &DisplayDef,
    placed_index: usize,
    room_width: f32,
) -> WorldPoint {
    let columns = display.capacity.min(5).max(1);
    let row = placed_index / columns;
    let column = placed_index % columns;
    let spacing_x = display.w / (columns as f32 + 1.0);
    let row_count = ((display.capacity + columns - 1) / columns).max(1);
    let spacing_y = display.h / (row_count as f32 + 1.0);
    let x = display.x + spacing_x * (column as f32 + 1.0);
    let y = display.y + spacing_y * (row as f32 + 1.0);

    WorldPoint {
        x: x.clamp(0.4, room_width - 0.4),
        y,
    }
}

#[cfg(test)]
mod tests;
