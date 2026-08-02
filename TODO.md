# TODO — Toybox After Hours

## Repair flow at scale

- Balance pass now that ~12% of 4000 toys spawn broken: mistake penalty, timer, and repair pacing, all tuned through `assets/data/*.json`.
- Repair parts render per *category* but not per *identity* — every plush head is the same model, so a broken Bear and a broken Octopus are indistinguishable while split. Decide whether identity-level parts are worth 50×2 renderers or whether identity-derived detail on the existing 10 is enough.

## Game loop and progression

- Rework the finish condition and finish screen around zone milestones now that per-zone completion exists (`GameSession::zone_progress`), and decide an intended run length. Note a zone caps at ~88% until its broken toys are repaired — milestones need to account for that or no zone ever reads done.
- Give a run a clear arc: an opening-time deadline with a score screen (toys shelved, repairs, mistakes, zones done) plus a relaxed untimed mode.

## Polish

- Per-zone procedural ambient variety (accent lighting tints, night sky through the window, checkout clutter) — no textures.
- Refresh the README and controls once the full loop lands, and regenerate `catalog_thumbnail.png` if the title screen changes.

## Engineering

- Deterministic replay tests at the 4000-toy scale for sorting, scoring, mistake penalties, timer acceleration, and completion goals.
- Scenario fixtures for beginner, mid-upgrade, and high-pressure sorting runs.
- Balance the five tools against real play: costs and unlock thresholds were picked to fit inside one run's credits, not from measured pacing. The Sorting Trolley in particular (carry 1 -> 3) is untested against the mistake/timer economy.
- Separate 3D scene rendering from session mutation so camera and visual effects cannot affect scoring.
- Fold the release-build capture path back into the shared `macroquad-toolkit/scripts/capture_ui.ps1` (a `-Release` switch) so other large games do not each need a local `capture_scene.ps1`.
