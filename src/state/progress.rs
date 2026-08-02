//! Per-zone completion — the "which aisle still needs work" readout.

use super::{toy_matches_display, GameSession, RepairPartKind};
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

    /// A zone is restored when every slot in it holds the right toy. Note this
    /// cannot happen until the zone's broken toys are rejoined: a split toy is
    /// two parts that match no display, so a zone with breaks caps below 100%
    /// however diligently its whole toys are shelved.
    pub fn is_restored(self) -> bool {
        self.placed >= self.capacity
    }
}

/// Everything the score screen reports, gathered once so the panel does not
/// recompute a zone sweep per row.
#[derive(Debug, Clone)]
pub struct ShiftSummary {
    pub zones: Vec<(String, ZoneProgress)>,
    pub zones_restored: usize,
    pub zones_with_shelves: usize,
    pub toys_shelved: usize,
    pub toy_count: usize,
    pub repairs: u32,
    pub breaks_total: u32,
    pub mistakes: u32,
    pub elapsed_seconds: f32,
}

impl ShiftSummary {
    /// 0.0..=1.0 over the whole shop, by slots filled with the right toy.
    pub fn completion(&self) -> f32 {
        if self.toy_count == 0 {
            return 1.0;
        }
        (self.toys_shelved as f32 / self.toy_count as f32).clamp(0.0, 1.0)
    }

    /// A letter for the run. Completion decides the band, because shelving the
    /// shop is the job; mistakes only pull a finished run down off the top
    /// grade, so a careless perfect clear is not the same as a clean one.
    pub fn grade(&self) -> &'static str {
        let completion = self.completion();
        if completion >= 1.0 {
            return if self.mistakes == 0 { "S" } else { "A" };
        }
        match completion {
            c if c >= 0.90 => "A",
            c if c >= 0.75 => "B",
            c if c >= 0.50 => "C",
            c if c >= 0.25 => "D",
            _ => "E",
        }
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

    /// Names of the aisles currently fully restored. Compared across a
    /// placement to spot the moment one lands. Zones without shelves are
    /// excluded: Checkout and the backroom are always trivially "done", and
    /// announcing them would fire on the first toy of the run.
    pub fn restored_zone_names(&self, data: &GameData) -> Vec<String> {
        self.zone_progress(data)
            .into_iter()
            .filter(|zone| zone.has_displays() && zone.is_restored())
            .map(|zone| data.layout.zones[zone.zone_index].name.clone())
            .collect()
    }

    pub fn shift_summary(&self, data: &GameData) -> ShiftSummary {
        let zones: Vec<(String, ZoneProgress)> = self
            .zone_progress(data)
            .into_iter()
            .filter(|zone| zone.has_displays())
            .map(|zone| (data.layout.zones[zone.zone_index].name.clone(), zone))
            .collect();

        ShiftSummary {
            zones_restored: zones.iter().filter(|(_, zone)| zone.is_restored()).count(),
            zones_with_shelves: zones.len(),
            toys_shelved: zones.iter().map(|(_, zone)| zone.placed).sum(),
            zones,
            toy_count: data.config.toy_count,
            repairs: self.player.repairs,
            breaks_total: self.total_breaks(),
            mistakes: self.player.mistakes,
            elapsed_seconds: self.player.elapsed_seconds,
        }
    }

    /// How many toys the shop broke this run — repaired ones included, since a
    /// mended toy is no longer a break but still counts toward the denominator
    /// of "repairs done".
    fn total_breaks(&self) -> u32 {
        let outstanding = self
            .toys
            .iter()
            .filter(|toy| toy.repair_part_kind() == Some(RepairPartKind::Head))
            .count() as u32;
        outstanding + self.player.repairs
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
