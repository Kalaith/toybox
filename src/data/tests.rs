use super::*;

#[test]
fn embedded_data_loads() {
    let data = GameData::load().unwrap();
    let total_capacity: usize = data.displays.iter().map(|display| display.capacity).sum();

    assert_eq!(data.config.game_name, "toybox_after_hours");
    assert_eq!(data.displays.len(), 20);
    assert_eq!(total_capacity, data.config.toy_count);
}

/// A tool nobody can reach is dead content, and a duplicate id is bought
/// once but owned twice — `has_upgrade` matches by id.
#[test]
fn every_tool_is_reachable_and_uniquely_named() {
    /// Roughly two wrapped lines at the shop's 14px description size.
    const DESCRIPTION_BUDGET: usize = 125;

    let data = GameData::load().unwrap();
    assert!(!data.upgrades.is_empty());

    let mut seen = std::collections::HashSet::new();
    let mut total_cost = 0;
    for upgrade in &data.upgrades {
        assert!(
            seen.insert(upgrade.id.as_str()),
            "duplicate id {}",
            upgrade.id
        );
        assert!(
            upgrade.unlock_completed_displays <= data.displays.len(),
            "{} unlocks at {} completed displays but only {} exist",
            upgrade.id,
            upgrade.unlock_completed_displays,
            data.displays.len()
        );
        // The shop row gives a description two wrapped lines. Rendered width
        // cannot be measured without a GL context, so this guards the
        // proxy — but it is the measure that broke: four of five tools once
        // ended mid-word, and the Sorting Trolley lost the half explaining
        // how to load it, which the shop is the only place to learn.
        // Re-capture `tool_shop` after changing any of these.
        assert!(
            upgrade.description.len() <= DESCRIPTION_BUDGET,
            "{} has a {}-character description; the shop row fits about {}",
            upgrade.id,
            upgrade.description.len(),
            DESCRIPTION_BUDGET
        );
        total_cost += upgrade.cost;
    }

    // Credits come one per completed display, so the whole shop has to be
    // affordable inside a single run or the last tools never sell.
    assert!(
        total_cost <= data.displays.len(),
        "tools cost {total_cost} credits but a run yields at most {}",
        data.displays.len()
    );
}
