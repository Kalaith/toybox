//! Tool-shop economy: which upgrades are owned, what they cost, and how the
//! credits earned from completed displays are spent.

use super::{GameSession, ToolPurchaseResult, SINGLE_CARRY_LIMIT};
use crate::data::{GameConfig, GameData};

pub(super) const TOY_SCANNER_ID: &str = "toy_scanner";
const LEGACY_TAG_LANTERN_ID: &str = "tag_lantern";

impl GameSession {
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
