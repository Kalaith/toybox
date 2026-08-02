//! Per-zone completion — the "which aisle still needs work" readout.

use super::{toy_matches_display, GameSession};
use crate::data::{DisplayDef, GameData};
use std::collections::HashMap;

/// How far one zone's displays are from fully stocked.
///
/// Every toy belongs to exactly one display (by category and theme) and every
/// display sits in exactly one zone, so each zone's denominator is fixed for
/// the whole run — unlike counting loose toys, which drifts as the player
/// carries things across the shop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneProgress {
    pub zone_index: usize,
    pub placed: usize,
    pub capacity: usize,
}

impl ZoneProgress {
    /// 0.0..=1.0. A zone with no displays has nothing to tidy, so it reads full
    /// rather than reading as permanently 0% work done.
    pub fn fraction(self) -> f32 {
        if self.capacity == 0 {
            1.0
        } else {
            self.placed as f32 / self.capacity as f32
        }
    }

    pub fn has_displays(self) -> bool {
        self.capacity > 0
    }
}

impl GameSession {
    /// One entry per zone, in `data.layout.zones` order.
    pub fn zone_progress(&self, data: &GameData) -> Vec<ZoneProgress> {
        let mut progress: Vec<ZoneProgress> = (0..data.layout.zones.len())
            .map(|zone_index| ZoneProgress {
                zone_index,
                placed: 0,
                capacity: 0,
            })
            .collect();

        let mut by_display: HashMap<&str, (usize, &DisplayDef)> = HashMap::new();
        for display in &data.displays {
            let Some(zone_index) = zone_index_of(data, display) else {
                continue;
            };
            progress[zone_index].capacity += display.capacity;
            by_display.insert(display.id.as_str(), (zone_index, display));
        }

        for toy in &self.toys {
            let Some(display_id) = toy.placed_display_id.as_deref() else {
                continue;
            };
            let Some(&(zone_index, display)) = by_display.get(display_id) else {
                continue;
            };
            // Wrongly shelved toys sit on a display without counting toward it.
            if toy_matches_display(toy, display) {
                progress[zone_index].placed += 1;
            }
        }

        progress
    }

    /// The zone the player is standing in, if any.
    pub fn current_zone_index(&self, data: &GameData) -> Option<usize> {
        let position = self.player.position;
        data.layout
            .zones
            .iter()
            .position(|zone| contains(zone.x, zone.y, zone.w, zone.h, position.x, position.y))
    }
}

/// Which zone a display belongs to, tested at its centre so a display flush
/// against a zone edge still lands inside it.
fn zone_index_of(data: &GameData, display: &DisplayDef) -> Option<usize> {
    let x = display.x + display.w * 0.5;
    let y = display.y + display.h * 0.5;
    data.layout
        .zones
        .iter()
        .position(|zone| contains(zone.x, zone.y, zone.w, zone.h, x, y))
}

fn contains(zone_x: f32, zone_y: f32, w: f32, h: f32, x: f32, y: f32) -> bool {
    x >= zone_x && x <= zone_x + w && y >= zone_y && y <= zone_y + h
}
