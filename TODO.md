# TODO — Toybox After Hours

## Repair flow at scale

- Show from a distance what is waiting on each bench (procedural indicator on the bench itself).
- Add `InteractionPreview` guidance for the "matching part is somewhere else" case.
- Give the Toy Scanner (or a new tool) the ability to locate a carried part's counterpart.
- Balance pass now that ~12% of 4000 toys spawn broken: mistake penalty, timer, and repair pacing, all tuned through `assets/data/*.json`.
- Repair parts still render as the generic head/body model for every category.

## Game loop and progression

- Per-zone and overall completion percentages on the HUD and minimap; rework the finish condition and finish screen around zone milestones and decide an intended run length.
- Expand the tool shop — `upgrades.json` holds only `toy_scanner`. Add 4–6 tools (carry capacity, speed, part compass, sort hint, mistake forgiveness) with real implementations in `state/` and costs tuned to zone pacing.
- Give a run a clear arc: an opening-time deadline with a score screen (toys shelved, repairs, mistakes, zones done) plus a relaxed untimed mode.
- Audit save migration: a pre-expansion save must load through `migrate_save_value` or fail gracefully, covered by a test in `state/tests.rs`.

## Polish

- Per-zone procedural ambient variety (accent lighting tints, night sky through the window, checkout clutter) — no textures.
- Refresh the README and controls once the full loop lands, and regenerate `catalog_thumbnail.png` if the title screen changes.

## Engineering

- Deterministic replay tests at the 4000-toy scale for sorting, scoring, mistake penalties, timer acceleration, and completion goals.
- Scenario fixtures for beginner, mid-upgrade, and high-pressure sorting runs.
- Validate upgrade availability and challenge metadata before a run starts so new toy types cannot break progression.
- Separate 3D scene rendering from session mutation so camera and visual effects cannot affect scoring.
- `src/toys/library.rs`, `src/ui/hud.rs`, and `src/state.rs` all sit above 700 lines; split them before the next feature lands in any of them.
