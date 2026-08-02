//! Runtime toy-store state, save data, and save migration helpers.

use crate::data::{DisplayDef, GameData, ToyCategory};
use crate::toys::ToySpawnPose;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

mod collision;
mod interactions;
mod persistence;
mod repair;
mod spatial;
mod spawn;
mod tools;

use collision::{keep_off_fixtures, position_blocked};
use spawn::build_toys;

pub use persistence::{migrate_save_value, SaveData};
pub use spatial::ToySpatialGrid;

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
    pub bench_id: Option<String>,
    #[serde(default)]
    pub wrong_marker_seconds: f32,
    #[serde(default)]
    pub repair_state: RepairState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
#[derive(Default)]
pub enum RepairState {
    #[default]
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

#[derive(Debug, Clone)]
pub struct GameSession {
    pub player: PlayerState,
    pub toys: Vec<ToyState>,
    pub displays: Vec<DisplayState>,
    pub unlocked_upgrade_ids: Vec<String>,
    pub phase: GamePhase,
    spatial: ToySpatialGrid,
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

const SINGLE_CARRY_LIMIT: usize = 1;

impl GameSession {
    pub const MAX_LOOK_PITCH: f32 = 1.18;
    pub const WRONG_MARKER_SECONDS: f32 = 2.5;

    pub fn new(data: &GameData) -> Self {
        let config = &data.config;
        let toys = build_toys(data);
        let mut spatial = ToySpatialGrid::new(
            config.room_width,
            config.room_height,
            config.spatial_cell_size,
        );
        spatial.rebuild(&toys);
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
            spatial,
        }
    }

    pub fn spatial(&self) -> &ToySpatialGrid {
        &self.spatial
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

    pub fn move_player(&mut self, direction: Vec2, data: &GameData, dt: f32) {
        if self.phase != GamePhase::Playing || direction.length_squared() == 0.0 {
            return;
        }

        let config = &data.config;
        let forward = vec2(self.player.yaw.cos(), self.player.yaw.sin());
        let right = vec2(-forward.y, forward.x);
        let world_direction = right * direction.x + forward * direction.y;
        if world_direction.length_squared() == 0.0 {
            return;
        }

        let step = world_direction.normalize() * config.player_speed * dt;
        let current = self.player.position.to_vec2();
        let clamp_x = |x: f32| x.clamp(0.45, config.room_width - 0.45);
        let clamp_y = |y: f32| y.clamp(0.45, config.room_height - 0.45);

        // Legacy saves can wake up inside a newly added fixture: let them
        // walk freely until clear instead of pinning them in place.
        if position_blocked(current, data) {
            self.player.position = WorldPoint::from_vec2(vec2(
                clamp_x(current.x + step.x),
                clamp_y(current.y + step.y),
            ));
            return;
        }

        // Axis-separated moves so the player slides along fixture edges.
        let mut next = current;
        let candidate_x = vec2(clamp_x(current.x + step.x), next.y);
        if !position_blocked(candidate_x, data) {
            next.x = candidate_x.x;
        }
        let candidate_y = vec2(next.x, clamp_y(current.y + step.y));
        if !position_blocked(candidate_y, data) {
            next.y = candidate_y.y;
        }
        self.player.position = WorldPoint::from_vec2(next);
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
        self.toys[toy_index].bench_id = None;
        self.toys[toy_index].wrong_marker_seconds = 0.0;
        self.toys[toy_index].position = drop_position;
        self.spatial.sync_toy(toy_index, &self.toys[toy_index]);
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
        let off_fixtures = keep_off_fixtures(clamped, data);

        WorldPoint {
            x: off_fixtures.x.clamp(0.65, config.room_width - 0.65),
            y: off_fixtures.y.clamp(0.65, config.room_height - 0.65),
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
                    toy.bench_id = None;
                    toy.wrong_marker_seconds = 0.0;
                    toy.position = self.player.position;
                }
            }
            self.spatial.rebuild(&self.toys);
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

fn default_player_yaw() -> f32 {
    -std::f32::consts::FRAC_PI_2
}

pub fn display_slot_position(
    display: &DisplayDef,
    placed_index: usize,
    room_width: f32,
) -> WorldPoint {
    let columns = display.capacity.clamp(1, 5);
    let row = placed_index / columns;
    let column = placed_index % columns;
    let spacing_x = display.w / (columns as f32 + 1.0);
    let row_count = display.capacity.div_ceil(columns).max(1);
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
