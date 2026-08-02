# TODO — Toybox After Hours

## Repair flow at scale

- **Repairs are close to unreachable as tuned.** A replay closer working nearest-first still completes *zero* repairs in 400 actions and defers 58-103 parts. Benches no longer brick, and the closer now routes a carried part to whichever bench already holds its other half, but neither helps: heads scatter into a *different zone* from their body, so over a 400-action sample of a 4000-toy shop the two halves are essentially never both to hand. This needs a design decision, not another fix:
  - shorten the scatter (keep a head in the body's zone, or an adjacent one), **or**
  - let one bench hold parts from several breaks (raise `capacity` in `layout.json`), **or**
  - have the scanner route the player to a pair rather than only reporting where the far half is.
  Until one is chosen, ~12% of the shop cannot be shelved at all.
- Mistake penalty and timer are still untuned against `assets/data/*.json`.
- Repair parts render per *category* but not per *identity* — every plush head is the same model, so a broken Bear and a broken Octopus are indistinguishable while split. Decide whether identity-level parts are worth 50×2 renderers or whether identity-derived detail on the existing 10 is enough.

## Game loop and progression

- Rework the finish condition and finish screen around zone milestones now that per-zone completion exists (`GameSession::zone_progress`). Note a zone caps at ~88% until its broken toys are repaired.
- **Decide an intended run length — the current one is far too long.** Replay measures ~342 toys shelved per 400 actions at 18.8 min; the full 4000-toy shop extrapolates to roughly 3.7 hours, and no display completes in the first 400 actions. Either cut `toy_count`, raise display throughput, or make a shift a slice of the shop.
- Give a run a clear arc: an opening-time deadline with a score screen (toys shelved, repairs, mistakes, zones done) plus a relaxed untimed mode.

## Polish

- Per-zone procedural ambient variety (accent lighting tints, night sky through the window, checkout clutter) — no textures.
- Refresh the README and controls once the full loop lands, and regenerate `catalog_thumbnail.png` if the title screen changes.

## Engineering

- Balance the five tools against measured pacing. `state/tests/replay.rs` now prints a per-loadout table; over 400 actions the Sorting Trolley halves both walking (6482m -> 2662m) and clock (39.8 -> 18.8 min), and Grippy Sneakers takes another 18% off. The trolley may be too strong for 2 credits.
- Separate 3D scene rendering from session mutation so camera and visual effects cannot affect scoring.
- Fold the release-build capture path back into the shared `macroquad-toolkit/scripts/capture_ui.ps1` (a `-Release` switch) so other large games do not each need a local `capture_scene.ps1`.
