//! Tool-shop economy: which upgrades are owned, what they cost, and how the
//! credits earned from completed displays are spent.

use super::{toy_matches_display, GameSession, ToolPurchaseResult, ToyState};
use crate::data::{GameData, UpgradeEffect};

pub(super) const TOY_SCANNER_ID: &str = "toy_scanner";
const LEGACY_TAG_LANTERN_ID: &str = "tag_lantern";
pub const STOCKROOM_SPOTLIGHT_NAME: &str = "Stockroom Spotlight";
pub const STOCKROOM_SPOTLIGHT_COST: usize = 1;
pub const STOCKROOM_SPOTLIGHT_SECONDS: f32 = 60.0;
pub const STOCKROOM_SPOTLIGHT_MAX_SECONDS: f32 = 180.0;

impl GameSession {
    /// The effects of every tool the player currently owns.
    fn owned_effects<'a>(&'a self, data: &'a GameData) -> impl Iterator<Item = UpgradeEffect> + 'a {
        data.upgrades
            .iter()
            .filter(|upgrade| self.has_upgrade(&upgrade.id))
            .map(|upgrade| upgrade.effect)
    }

    /// How many toys the player can hold. Carry tools do not stack — owning a
    /// bigger trolley replaces a smaller one rather than adding to it.
    pub fn carry_limit(&self, data: &GameData) -> usize {
        self.owned_effects(data)
            .filter_map(|effect| match effect {
                UpgradeEffect::CarryLimit { toys } => Some(toys),
                _ => None,
            })
            .max()
            .unwrap_or(data.config.starting_carry_limit)
            .max(1)
    }

    /// The tool currently granting the carry limit, or `None` bare-handed.
    ///
    /// Taken from `upgrades.json` rather than written out, because the message
    /// that used it said "Sorting cart is full" — a name the game does not use
    /// (it is the Sorting *Trolley*), and one that collides with the Cart
    /// Blocks toy. Worse, `InventoryFull` fires at the *starting* limit of one,
    /// so that message named a tool the player had never bought.
    pub fn carry_tool_name<'a>(&self, data: &'a GameData) -> Option<&'a str> {
        data.upgrades
            .iter()
            .filter(|upgrade| self.has_upgrade(&upgrade.id))
            .filter_map(|upgrade| match upgrade.effect {
                UpgradeEffect::CarryLimit { toys } => Some((toys, upgrade.name.as_str())),
                _ => None,
            })
            .max_by_key(|(toys, _)| *toys)
            .map(|(_, name)| name)
    }

    pub fn speed_multiplier(&self, data: &GameData) -> f32 {
        self.owned_effects(data)
            .filter_map(|effect| match effect {
                UpgradeEffect::Speed { multiplier } => Some(multiplier),
                _ => None,
            })
            .fold(1.0, f32::max)
    }

    /// How far the player can reach to pick a toy up, in world units.
    pub fn interaction_reach(&self, data: &GameData) -> f32 {
        let multiplier = self
            .owned_effects(data)
            .filter_map(|effect| match effect {
                UpgradeEffect::Reach { multiplier } => Some(multiplier),
                _ => None,
            })
            .fold(1.0, f32::max);
        data.config.interaction_radius * multiplier
    }

    /// Placement guards granted by owned tools. Used to initialise old saves;
    /// live play consumes `player.mistake_guards_remaining` instead.
    pub fn forgiven_mistakes(&self, data: &GameData) -> u32 {
        self.owned_effects(data)
            .filter_map(|effect| match effect {
                UpgradeEffect::MistakeForgiveness { mistakes } => Some(mistakes),
                _ => None,
            })
            .sum()
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

    pub fn scanner_enabled(&self, data: &GameData) -> bool {
        self.owned_effects(data)
            .any(|effect| matches!(effect, UpgradeEffect::Scanner))
    }

    /// The closest compatible display with an unoccupied slot. All four
    /// fixtures in a category accept the same stock, so the scanner is route
    /// guidance rather than a claim that a toy has one arbitrary true shelf.
    pub fn recommended_display_index(&self, data: &GameData, toy: &ToyState) -> Option<usize> {
        let player = self.player.position.to_vec2();
        data.displays
            .iter()
            .enumerate()
            .filter(|(_, display)| {
                toy_matches_display(toy, display)
                    && self
                        .toys
                        .iter()
                        .filter(|placed| {
                            placed.placed_display_id.as_deref() == Some(display.id.as_str())
                        })
                        .count()
                        < display.capacity
            })
            .min_by(|(_, left), (_, right)| {
                let left_center =
                    macroquad::prelude::vec2(left.x + left.w * 0.5, left.y + left.h * 0.5);
                let right_center =
                    macroquad::prelude::vec2(right.x + right.w * 0.5, right.y + right.h * 0.5);
                left_center
                    .distance_squared(player)
                    .total_cmp(&right_center.distance_squared(player))
            })
            .map(|(index, _)| index)
    }

    pub fn all_tools_owned(&self, data: &GameData) -> bool {
        data.upgrades
            .iter()
            .all(|upgrade| self.has_upgrade(&upgrade.id))
    }

    pub fn stockroom_spotlight_active(&self) -> bool {
        self.player.stockroom_spotlight_seconds > 0.0
    }

    /// The nearest unfinished piece of floor work. The spotlight deliberately
    /// does not move or identify the toy's home; it only prevents the last few
    /// small objects from becoming a minimap pixel hunt.
    pub fn stockroom_spotlight_target(&self) -> Option<&ToyState> {
        let player = self.player.position.to_vec2();
        self.toys
            .iter()
            .filter(|toy| {
                !toy.is_held
                    && toy.placed_display_id.is_none()
                    && toy.bench_slot_index.is_none()
                    && !toy.is_consumed_repair_part()
            })
            .min_by(|left, right| {
                left.position
                    .to_vec2()
                    .distance_squared(player)
                    .total_cmp(&right.position.to_vec2().distance_squared(player))
            })
    }

    pub fn available_tool_credits(&self, data: &GameData) -> usize {
        self.completed_display_count()
            .saturating_sub(self.spent_tool_credits(data) + self.player.service_credits_spent)
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
        if let UpgradeEffect::MistakeForgiveness { mistakes } = upgrade.effect {
            self.player.mistake_guards_remaining = self
                .player
                .mistake_guards_remaining
                .saturating_add(mistakes);
            self.player.mistake_guards_initialized = true;
        }
        ToolPurchaseResult::Purchased {
            tool_name: upgrade.name.clone(),
            remaining_credits: self.available_tool_credits(data),
        }
    }

    pub fn purchase_stockroom_spotlight(&mut self, data: &GameData) -> ToolPurchaseResult {
        if !self.all_tools_owned(data) {
            return ToolPurchaseResult::NoToolsAvailable;
        }
        if self.player.stockroom_spotlight_seconds + f32::EPSILON >= STOCKROOM_SPOTLIGHT_MAX_SECONDS
        {
            return ToolPurchaseResult::ServiceAtCapacity {
                service_name: STOCKROOM_SPOTLIGHT_NAME,
                seconds_active: self.player.stockroom_spotlight_seconds,
            };
        }
        let available_credits = self.available_tool_credits(data);
        if available_credits < STOCKROOM_SPOTLIGHT_COST {
            return ToolPurchaseResult::NeedMoreCredits {
                tool_name: STOCKROOM_SPOTLIGHT_NAME.to_owned(),
                cost: STOCKROOM_SPOTLIGHT_COST,
                available_credits,
            };
        }

        self.player.service_credits_spent += STOCKROOM_SPOTLIGHT_COST;
        self.player.stockroom_spotlight_seconds = (self.player.stockroom_spotlight_seconds
            + STOCKROOM_SPOTLIGHT_SECONDS)
            .min(STOCKROOM_SPOTLIGHT_MAX_SECONDS);
        ToolPurchaseResult::ServicePurchased {
            service_name: STOCKROOM_SPOTLIGHT_NAME,
            seconds_active: self.player.stockroom_spotlight_seconds,
            remaining_credits: self.available_tool_credits(data),
        }
    }

    pub(super) fn newly_available_upgrades(
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
}
