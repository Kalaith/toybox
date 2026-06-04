//! Embedded game data and asset manifests for Toybox After Hours.

use macroquad_toolkit::assets::TextureConfig;
use macroquad_toolkit::data_loader::{load_embedded_json, load_embedded_json_labeled};
use serde::{Deserialize, Serialize};

const GAME_CONFIG_JSON: &str = include_str!("../assets/data/game_config.json");
const DISPLAYS_JSON: &str = include_str!("../assets/data/displays.json");
const UPGRADES_JSON: &str = include_str!("../assets/data/upgrades.json");
const TEXTURE_MANIFEST_JSON: &str = include_str!("../assets/data/texture_manifest.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_name: String,
    pub display_name: String,
    pub save_slot: String,
    pub version: String,
    pub room_width: f32,
    pub room_height: f32,
    pub toy_count: usize,
    pub starting_carry_limit: usize,
    pub player_speed: f32,
    pub interaction_radius: f32,
    pub mistake_penalty_seconds: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToyCategory {
    Plushies,
    TinyDragons,
    BuildingBlocks,
    ActionFigures,
    BoardGames,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayDef {
    pub id: String,
    pub name: String,
    pub category: ToyCategory,
    pub theme: String,
    pub capacity: usize,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub accent: [f32; 4],
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlock_completed_displays: usize,
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub config: GameConfig,
    pub displays: Vec<DisplayDef>,
    pub upgrades: Vec<UpgradeDef>,
    pub texture_manifest: Vec<TextureConfig>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        let config = load_embedded_json_labeled("game_config", GAME_CONFIG_JSON)?;
        let displays = load_embedded_json_labeled("displays", DISPLAYS_JSON)?;
        let upgrades = load_embedded_json_labeled("upgrades", UPGRADES_JSON)?;
        let texture_manifest = load_embedded_json(TEXTURE_MANIFEST_JSON)?;

        Ok(Self {
            config,
            displays,
            upgrades,
            texture_manifest,
        })
    }

    pub fn display_by_id(&self, id: &str) -> Option<&DisplayDef> {
        self.displays.iter().find(|display| display.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_data_loads() {
        let data = GameData::load().unwrap();
        let total_capacity: usize = data.displays.iter().map(|display| display.capacity).sum();

        assert_eq!(data.config.game_name, "toybox_after_hours");
        assert_eq!(data.displays.len(), 5);
        assert_eq!(data.upgrades.len(), 1);
        assert_eq!(total_capacity, data.config.toy_count);
    }
}
