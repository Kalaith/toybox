//! Runtime toy-store state, save data, and save migration helpers.

use crate::data::{DisplayDef, GameConfig, GameData, ToyCategory};
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub is_held: bool,
    pub placed_display_id: Option<String>,
    #[serde(default)]
    pub wrong_marker_seconds: f32,
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
    Placed {
        toy_name: String,
        display_name: String,
        completed_display: Option<String>,
        unlocked_upgrades: Vec<String>,
        finished: bool,
    },
    InventoryFull,
    NothingNearby,
}

#[derive(Debug, Clone)]
pub enum InteractionPreview {
    PlaceMatch,
    PlaceMismatch,
    Pickup {
        toy_name: String,
    },
    InventoryFull,
    NothingNearby,
    Finished,
}

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
                    x: config.room_width * 0.5,
                    y: config.room_height * 0.5,
                },
                yaw: default_player_yaw(),
                pitch: 0.0,
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

    pub fn carry_limit(&self, config: &GameConfig) -> usize {
        config.starting_carry_limit
    }

    pub fn has_upgrade(&self, upgrade_id: &str) -> bool {
        self.unlocked_upgrade_ids
            .iter()
            .any(|existing_id| existing_id == upgrade_id)
    }

    pub fn active_toy(&self) -> Option<&ToyState> {
        let toy_id = self
            .player
            .carried_toy_ids
            .get(self.player.active_carry_index)?;
        self.toys.iter().find(|toy| &toy.id == toy_id)
    }

    pub fn select_carried(&mut self, index: usize) {
        if index < self.player.carried_toy_ids.len() {
            self.player.active_carry_index = index;
        }
    }

    pub fn cycle_carried(&mut self) {
        if self.player.carried_toy_ids.is_empty() {
            self.player.active_carry_index = 0;
        } else {
            self.player.active_carry_index =
                (self.player.active_carry_index + 1) % self.player.carried_toy_ids.len();
        }
    }

    pub fn drop_active(&mut self) -> Option<String> {
        let toy_id = self.active_toy()?.id.clone();
        let toy_index = self.toys.iter().position(|toy| toy.id == toy_id)?;
        let toy_name = self.toys[toy_index].name.clone();

        self.toys[toy_index].is_held = false;
        self.toys[toy_index].placed_display_id = None;
        self.toys[toy_index].position = self.player.position;
        self.player.carried_toy_ids.retain(|id| id != &toy_id);
        self.normalize_active_carry();

        Some(toy_name)
    }

    pub fn interact(&mut self, data: &GameData) -> InteractionResult {
        if self.phase != GamePhase::Playing {
            return InteractionResult::NothingNearby;
        }

        if self.active_toy().is_some() {
            if let Some(display_index) = self.nearest_display_index(data) {
                return self.place_active_toy(display_index, data);
            }
        }

        if self.player.carried_toy_ids.len() >= self.carry_limit(&data.config) {
            return InteractionResult::InventoryFull;
        }

        if let Some(toy_index) = self.nearest_loose_toy_index(&data.config) {
            return self.pick_up_toy(toy_index);
        }

        InteractionResult::NothingNearby
    }

    pub fn interaction_preview(&self, data: &GameData) -> InteractionPreview {
        if self.phase != GamePhase::Playing {
            return InteractionPreview::Finished;
        }

        if let Some(active_toy) = self.active_toy() {
            if let Some(display_index) = self.nearest_display_index(data) {
                let display = &data.displays[display_index];
                if toy_matches_display(active_toy, display) {
                    return InteractionPreview::PlaceMatch;
                }
                return InteractionPreview::PlaceMismatch;
            }
        }

        if self.player.carried_toy_ids.len() >= self.carry_limit(&data.config) {
            return InteractionPreview::InventoryFull;
        }

        if let Some(toy_index) = self.nearest_loose_toy_index(&data.config) {
            return InteractionPreview::Pickup {
                toy_name: self.toys[toy_index].name.clone(),
            };
        }

        InteractionPreview::NothingNearby
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

    pub fn is_display_slot_filled(&self, display_id: &str, slot_number: usize) -> bool {
        self.toys.iter().any(|toy| {
            toy.slot_number == slot_number
                && toy
                    .placed_display_id
                    .as_deref()
                    .is_some_and(|placed_id| placed_id == display_id)
        })
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

        self.player
            .carried_toy_ids
            .retain(|toy_id| self.toys.iter().any(|toy| &toy.id == toy_id));
        self.normalize_active_carry();
        self.repair_player_view();
        self.refresh_display_completion(data);
        self.unlock_available_upgrades(data);
    }

    fn pick_up_toy(&mut self, toy_index: usize) -> InteractionResult {
        let toy_id = self.toys[toy_index].id.clone();
        let toy_name = self.toys[toy_index].name.clone();

        self.toys[toy_index].is_held = true;
        self.toys[toy_index].placed_display_id = None;
        self.player.carried_toy_ids.push(toy_id);
        self.player.active_carry_index = self.player.carried_toy_ids.len().saturating_sub(1);

        InteractionResult::PickedUp { toy_name }
    }

    fn place_active_toy(&mut self, display_index: usize, data: &GameData) -> InteractionResult {
        let display = &data.displays[display_index];
        let toy_id = match self.active_toy() {
            Some(toy) => toy.id.clone(),
            None => return InteractionResult::NothingNearby,
        };
        let toy_index = match self.toys.iter().position(|toy| toy.id == toy_id) {
            Some(index) => index,
            None => return InteractionResult::NothingNearby,
        };
        let toy_name = self.toys[toy_index].name.clone();

        let is_wrong_display = !toy_matches_display(&self.toys[toy_index], display);
        if is_wrong_display {
            self.player.mistakes += 1;
            self.player.elapsed_seconds += data.config.mistake_penalty_seconds;
        }

        self.toys[toy_index].is_held = false;
        self.toys[toy_index].placed_display_id = Some(display.id.clone());
        self.toys[toy_index].wrong_marker_seconds = if is_wrong_display {
            Self::WRONG_MARKER_SECONDS
        } else {
            0.0
        };
        self.toys[toy_index].position = display_slot_position(
            display,
            self.toys[toy_index].slot_number.saturating_sub(1),
            data.config.room_width,
        );
        self.player.carried_toy_ids.retain(|id| id != &toy_id);
        self.normalize_active_carry();

        let was_complete = self.is_display_complete(&display.id);
        if let Some(display_state) = self
            .displays
            .iter_mut()
            .find(|display_state| display_state.id == display.id)
        {
            if !display_state.placed_toy_ids.iter().any(|id| id == &toy_id) {
                display_state.placed_toy_ids.push(toy_id);
            }
        }

        self.refresh_display_completion(data);
        let completed_display = if !was_complete && self.is_display_complete(&display.id) {
            Some(display.name.clone())
        } else {
            None
        };
        let unlocked_upgrades = self.unlock_available_upgrades(data);
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
            completed_display,
            unlocked_upgrades,
            finished,
        }
    }

    fn nearest_loose_toy_index(&self, config: &GameConfig) -> Option<usize> {
        let player = self.player.position.to_vec2();
        let max_distance_sq = config.interaction_radius * config.interaction_radius;

        self.toys
            .iter()
            .enumerate()
            .filter(|(_, toy)| !toy.is_held && toy.placed_display_id.is_none())
            .filter_map(|(index, toy)| {
                let distance_sq = toy.position.to_vec2().distance_squared(player);
                (distance_sq <= max_distance_sq).then_some((index, distance_sq))
            })
            .min_by(|(_, left), (_, right)| left.total_cmp(right))
            .map(|(index, _)| index)
    }

    fn nearest_display_index(&self, data: &GameData) -> Option<usize> {
        let player = self.player.position.to_vec2();
        let max_distance_sq = data.config.interaction_radius * data.config.interaction_radius;

        data.displays
            .iter()
            .enumerate()
            .filter_map(|(index, display)| {
                let nearest_point = vec2(
                    player.x.clamp(display.x, display.x + display.w),
                    player.y.clamp(display.y, display.y + display.h),
                );
                let distance_sq = nearest_point.distance_squared(player);
                (distance_sq <= max_distance_sq).then_some((index, distance_sq))
            })
            .min_by(|(_, left), (_, right)| left.total_cmp(right))
            .map(|(index, _)| index)
    }

    fn refresh_display_completion(&mut self, data: &GameData) {
        for display_state in &mut self.displays {
            if let Some(display) = data.display_by_id(&display_state.id) {
                display_state.placed_toy_ids.retain(|toy_id| {
                    self.toys.iter().any(|toy| {
                        &toy.id == toy_id
                            && toy
                                .placed_display_id
                                .as_deref()
                                .is_some_and(|placed_id| placed_id == display.id)
                    })
                });
                display_state.is_complete = display_state.placed_toy_ids.len() >= display.capacity;
            }
        }
    }

    fn unlock_available_upgrades(&mut self, data: &GameData) -> Vec<String> {
        let completed_count = self.completed_display_count();
        let mut newly_unlocked = Vec::new();

        for upgrade in &data.upgrades {
            let should_unlock = completed_count >= upgrade.unlock_completed_displays;
            let already_unlocked = self.has_upgrade(&upgrade.id);
            if should_unlock && !already_unlocked {
                self.unlocked_upgrade_ids.push(upgrade.id.clone());
                newly_unlocked.push(upgrade.name.clone());
            }
        }

        newly_unlocked
    }

    fn normalize_active_carry(&mut self) {
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
    toy.category == display.category && toy.theme == display.theme
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
    let mut toys = Vec::with_capacity(data.config.toy_count);

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
                is_held: false,
                placed_display_id: None,
                wrong_marker_seconds: 0.0,
            });
        }
    }

    toys
}

fn toy_name(display: &DisplayDef, slot_number: usize) -> String {
    match display.category {
        ToyCategory::Plushies => format!("{} Plush #{slot_number:02}", display.theme),
        ToyCategory::TinyDragons => format!("Tiny Dragon {} #{slot_number:02}", display.theme),
        ToyCategory::BuildingBlocks => format!("{} Block Set #{slot_number:02}", display.theme),
        ToyCategory::ActionFigures => format!("{} Figure #{slot_number:02}", display.theme),
        ToyCategory::BoardGames => format!("{} Game Box #{slot_number:02}", display.theme),
    }
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
