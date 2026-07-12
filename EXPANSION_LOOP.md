# Toybox After Hours — Expansion Loop

You are one iteration of a recurring loop expanding **Toybox After Hours** from a small demo into a full game. Each invocation completes exactly **ONE roadmap task** end-to-end and commits it to `master`. Do not start a second task in the same iteration.

## End-state vision

- A large, multi-zone toy store: themed aisles/zones, checkout area, backroom — big enough that navigation and wayfinding matter.
- **4000+ toys scattered on the floor** at the start of a run, with a meaningful fraction (~10–15%) spawning broken into head/body parts scattered separately.
- Solid performance at that scale: 60 FPS native, playable in WebGL. This requires culling, LOD, and spatial indexing — brute force will not survive 4000 toys.
- A real game loop at scale: per-zone progress, a richer tool shop, and a satisfying finish — not just "shelve all 100."
- **All graphics stay procedural** (macroquad primitives + procedurally drawn textures like the existing wood). No image assets for toys or environment. The title texture is the only allowed image.

## Per-iteration procedure

1. **Orient.** Read this file's roadmap below and `git log --oneline -10`. If the working tree has uncommitted changes left by a previous iteration, verify them (step 4 gates) and commit them before anything else.
2. **Pick the first unchecked task** in the roadmap, top to bottom. Phases are ordered by dependency — do not skip ahead (in particular, do not raise `toy_count` past a perf gate that hasn't been done). If a task is too large for one sitting, split it into sub-checkboxes in this file and complete the first.
3. **Implement.** Respect every constraint in the section below. Prefer extending `assets/data/*.json` over hardcoding; prefer extending `macroquad-toolkit` patterns already in use.
4. **Gate — all must pass before committing:**
   ```powershell
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt
   ```
   For any task that touches rendering, spawning, or toy scale: also `cargo build --release` and confirm via the FPS overlay (Phase 0 task) that the frame-time budget still holds at the current `toy_count`. If a scale increase drops native FPS below 60, revert the scale increase, and instead add a perf task to the roadmap describing what must be optimized first.
5. **Record progress.** Check off the completed task in this file and add one line to the Log section at the bottom: date, task, and any decision the next iteration needs to know.
6. **Commit to `master`.** One focused commit including the roadmap update, message describing the player-visible or structural change (e.g. `Add spatial grid for toy queries and interaction targeting`). Never commit with failing gates. Do not push unless a remote push has been explicitly set up as part of the workflow.
7. **Stop.** End the iteration after one committed task. If every roadmap box is checked, say the expansion is complete and stop the loop instead of inventing new work.

## Hard constraints (repo rules — non-negotiable)

- **Procedural graphics only** for toys and environment, as today.
- **Data-driven:** balance, layout, content, and text live in `assets/data/*.json` loaded through `data.rs`. New tunables go in JSON, not Rust constants.
- **Deterministic generation:** toy spawning, broken-toy selection, and scatter positions must be deterministic (hash/index-based like `toys/library.rs`, or a seeded state-owned RNG). Two runs of a fresh game must produce identical stores.
- **Invariant (test-enforced):** sum of display capacities in `displays.json` == `game_config.toy_count`. Always change them together and keep the test meaningful.
- **Save compatibility:** new fields on `ToyState`/`PlayerState`/`SaveData` need `#[serde(default)]`; shape changes require bumping `game_config.version` and extending `migrate_save_value` in `state.rs`. An old save must load or fail gracefully — never panic.
- **UI never mutates state.** New interactions are new `UiAction` variants dispatched through `Game::apply_action`; new session outcomes are new `InteractionResult`/`InteractionPreview` variants.
- **800-line hard limit per `.rs` file** (soft target 200–400). Approaching it means extracting a sibling module — `foo.rs` + `foo/` children, never `mod.rs`. `state.rs` (774) and `ui/hud.rs` (737) are already near the limit: if you touch them, restructure as part of the task.
- No unused code, no `_`-prefixed dead fields, no new dependencies unless they remove real complexity.
- Never edit the synced shared docs (`AGENTS.md`, `CODE_STANDARDS.md`, `MACROQUAD_TOOLKIT.md`, `GAME_DEVELOPMENT_GUIDE.md`) and never write under `D:\xampp\htdocs`.
- Run the full `.\publish.ps1` **only** at the phase-boundary tasks that say so — not every iteration.

## Roadmap

### Phase 0 — Instrumentation & performance foundation
*(Nothing scales past 100 toys until this phase is done.)*

- [x] **FPS/frame-time debug overlay**, toggled with F3: FPS, frame ms, drawn-toy count vs total, player position/zone. Config flag in `game_config.json` to enable the toggle.
- [x] **Spatial grid for toys** in `state/`: uniform grid over the room keyed by cell → toy indices, kept in sync on pickup/drop/place. All proximity and look-target queries in `state/interactions.rs` and HUD preview use it — no full-`Vec` linear scans in per-frame paths.
- [x] **Render culling for loose toys**: distance cutoff + view-direction (dot-product) culling in `ui/scene3d.rs`, driven by the spatial grid.
- [x] **Toy LOD**: beyond a configurable distance, draw a toy as a single cheap primitive in its color; full per-identity detail only near the player. Distances in `game_config.json`.
- [ ] **Perf: cheap sphere primitives** — toy identity modules call `draw_sphere` (macroquad default 16×16 tessellation, re-generated per call per frame). Add a low-poly sphere helper in `toys/library.rs` (`draw_sphere_ex` with ~8 rings/slices, fewer at LOD range) and switch all toy modules to it. Re-measure with `TOYBOX_BENCH_SECONDS`.
- [ ] **Perf: eliminate per-toy batch flushes** — `draw_loose_toy_3d` wraps every loose toy in `push_model_matrix`/`pop_model_matrix`, which flushes the render batch per toy (~hundreds of flushes/frame at 500 toys). Apply spawn-pose orientation without a per-toy matrix push (rotate primitive offsets in code), or matrix-push only within a short full-detail radius. Re-measure.
- [ ] **Perf gate A:** temporarily raise `toy_count` to 500 (with matching display capacity — add filler capacity if displays don't exist yet), confirm 60 FPS native via `TOYBOX_BENCH_SECONDS` bench, then keep 500 as the new baseline. Log measured FPS. Re-tune `toy_render_distance`/`toy_lod_distance` here if needed.

### Phase 1 — Data-driven store layout

- [ ] **Extract layout to `assets/data/layout.json`**: walls, floor zones, fixture/bench positions, display placement anchors. `ui/environment.rs`/`ui/fixtures.rs` render from data; `state/repair.rs` bench position comes from data. Same visual result as today, zero hardcoded geometry.
- [ ] **Expand the store**: grow `room_width`/`room_height` substantially and lay out 5–7 themed zones (plush corner, dragon alcove, robot lab, board-game wall, block pit, checkout, backroom) with aisle shelving as obstacles. Player collision against walls/fixtures.
- [ ] **Wayfinding**: hanging zone signs (extend `ui/signs.rs`) and a simple procedural minimap on the HUD showing zones, displays, benches, and player. Large stores must stay navigable.
- [ ] **Multiple repair benches** placed via layout data; nearest-bench logic replaces the single-bench assumption.
- [ ] **Phase gate:** run `.\publish.ps1`, play a full loop in the browser build, fix anything broken, commit.

### Phase 2 — More toys, more displays, real scale

- [ ] **More toy identities**: grow toward ~10 identities per category (currently 5 each). Each is a new module in `src/toys/` + `ToyIdentity` variant + dispatch arm + `library.rs` roster entry. Batch 2–4 identities per iteration; keep silhouettes distinct so shelving-by-sight still works.
- [ ] **(Optional) one or two new categories** (e.g. `Vehicles`, `Puzzles`): new `ToyCategory` variant, displays, identities, HUD icon. Skip if the five categories already fill the store well.
- [ ] **Displays at scale**: many more display definitions across zones in `displays.json` (multiple displays per category, per zone). Capacity math stays exact against `toy_count`.
- [ ] **Perf gate B: `toy_count` → 1500.** 60 FPS native confirmed; scatter logic distributes toys across all zones (deterministic clusters/piles, not uniform noise). Log FPS.
- [ ] **Perf gate C: `toy_count` → 4000+.** 60 FPS native, WebGL still playable (log its FPS too). This is the headline target — if it fails, add and complete the needed optimization tasks (e.g. cheaper LOD primitive, draw batching, tighter culling) before raising the count.
- [ ] **Phase gate:** `.\publish.ps1`, verify the 4000-toy store in browser, update README player-facing description, commit.

### Phase 3 — Broken toys at scale

- [ ] **Data-driven breakage**: `broken_fraction` (and any per-category weights) in `game_config.json`; deterministic selection of which toys spawn broken. Head and body parts scatter to *different* zones so finding the match is a real task.
- [ ] **Repair flow at scale**: benches show what's waiting on them from a distance (procedural indicator); `InteractionPreview` guidance for "matching part is somewhere else" moments; Toy Scanner (or a new tool) helps locate a carried part's counterpart.
- [ ] **Balance pass**: with ~10–15% of 4000 toys broken, tune mistake penalty, timer, and repair pacing so a run stays fun rather than tedious. All values in JSON.

### Phase 4 — Full game loop & progression

- [ ] **Progress model for scale**: per-zone and overall completion percentages on HUD/minimap; per-display completion stays. Finish condition and finish screen reworked around zone milestones (shelving 4000 toys to 100% may be a marathon — decide and log the intended run length).
- [ ] **Expanded tool shop**: 4–6 new tools in `upgrades.json` (e.g. cart capacity +N, speed shoes, part magnet/compass, display auto-sort hint, mistake forgiveness), each actually implemented in `state/`. Costs/unlocks tuned to zone-completion pacing.
- [ ] **Run structure**: a clear arc — e.g. opening-time deadline with score screen (toys shelved, repairs, mistakes, zones done) and a relaxed untimed mode. Keep it simple; this is a cozy cleanup game, not a roguelike.
- [ ] **Save migration audit**: an old-format save (pre-expansion) loads via `migrate_save_value` or fails gracefully with a clear notification. Add a migration test in `state/tests.rs`.

### Phase 5 — Polish & ship

- [ ] **Environment polish**: procedural ambient variety per zone (accent lighting tints, window/skylight night sky, checkout clutter) — cheap, no textures.
- [ ] **Deterministic replay tests** (from README's future-improvements list): sorting, scoring, mistake penalties, completion goals exercised in `state/tests.rs` at the 4000-toy scale.
- [ ] **README + controls refresh** to describe the full game; regenerate `catalog_thumbnail.png` from the title screen if it changed.
- [ ] **Final gate:** `.\publish.ps1`, full browser playthrough of at least one zone to completion, commit. Then declare the loop complete and stop.

## Log

*(One line per completed iteration: date — task — decisions the next iteration needs.)*

- 2026-07-12 — FPS debug overlay (F3, `debug_overlay_enabled` in game_config.json) — new `ui/debug_overlay.rs`; `draw_shop_scene` now returns `SceneStats { drawn_toys }` (loose+placed), extend it when culling lands. Zone readout deferred until Phase 1 zones exist; overlay draws only on the Playing screen, panel sits right of the HUD status panel.
- 2026-07-12 — Spatial grid (`state/spatial.rs`, `spatial_cell_size` in game_config.json) — `GameSession.spatial` (private field) tracks all non-held, non-consumed toys; mutation sites call `sync_toy`, load paths call `rebuild`. Loose-toy targeting, display-slot occupancy (placed toys sit exactly on slot positions — lookups rely on that invariant), and bench queries are all grid-driven. Next task (culling) needs read access from `ui/scene3d.rs`: add a pub accessor + `pub use spatial::ToySpatialGrid` then. Tests that hand-move toys must call `session.spatial.rebuild(&session.toys)`.
- 2026-07-12 — Render culling for loose toys — `draw_loose_toys` queries `session.spatial()` (new pub accessor) within `toy_render_distance`, then culls by `toy_always_draw_radius`/`toy_view_cull_min_dot` (all in game_config.json; 0.15 keeps an ~81° half-angle cone vs the ~44° actual half-FOV, so no edge pop). Placed-toy drawing still iterates displays — cull it when displays scale up (Phase 2). FPS unverifiable headless; overlay drawn-count now reflects culling.
- 2026-07-12 — Toy LOD (`toy_lod_distance: 9.0` in game_config.json) — `draw_toy_lod_3d` in toys.rs draws one flat-color cube; applied to loose AND placed toys past the distance. LOD skips the spawn-pose matrix push too (distant tumble invisible). Perf gate A next: raise toy_count to 500 + matching display capacity, confirm 60 FPS via overlay (needs an interactive run — ask the user to verify if headless).
- 2026-07-12 — Perf gate A attempt + bench harness — added `TOYBOX_BENCH_SECONDS=<n>` env mode (game.rs): boots into a fresh run, sweeps yaw, prints `BENCH toys= frames= avg_fps= worst_frame_ms=` to stdout and exits. Measured (release, native): 100 toys → 151.5 avg FPS; 500 toys → 30.8 avg FPS. Nearly linear cost ⇒ reverted to 100 per gate rule and queued two perf tasks (sphere tessellation, per-toy matrix-push batch flushes) before retrying the gate. Bench dt is clamped at 0.1s by main.rs so worst_frame_ms caps at 100.
